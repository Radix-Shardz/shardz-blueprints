#[cfg(test)]
mod shardz_tests {
    use scrypto::prelude::*;
    use test_engine::{env_args, global_package};
    use test_engine::environment::Environment;
    use test_engine::receipt_traits::Outcome;
    use test_engine::test_engine::TestEngine;

    use radix_shardz::shardz::{ShardNFT, ShardTicket, ShardType};

    global_package!(SHARDZ, ".");

    pub fn instantiate() -> TestEngine {
        let mut test_engine = TestEngine::new();

        test_engine.new_account("admin");
        test_engine.new_account("user1");
        test_engine.new_account("user2");
        test_engine.set_current_account("admin");

        test_engine.add_token(
            "admin badge",
            1,
            "resource_tdx_2_1t5xcs0ma3qcdp3q8k5ulzzq9vke8dp5ca80lngxj95cjvxsuqrzzp0",
            NetworkDefinition::stokenet(),
        );

        test_engine.add_global_package("shardz package", &SHARDZ);

        test_engine.new_component("shards comp", "Shardz", "instantiate_shardz", env_args!(*test_engine.current_account_address()));

        test_engine
    }

    #[test]
    fn test_instantiation() {
        let mut test_engine = instantiate();

        // Check that all resources have been created
        test_engine.get_resource("Shard");
        test_engine.get_resource("Shard NFT");
        test_engine.get_resource("Shard Ticket");

        // Check that the admin received 1000 Shards
        assert_eq!(test_engine.balance_of("admin", "Shard"), dec!(1000));
    }

    #[test]
    fn test_bond() {
        let mut test_engine = instantiate();

        test_engine.call_method("bond", env_args!(Environment::FungibleBucket("shard", dec!("3.23"))))
            .expect_commit_success();

        assert_eq!(test_engine.current_balance("Shard"), dec!(997));
        assert_eq!(test_engine.current_balance("Shard NFT"), dec!(0));

        let mut shard_tickets = test_engine.current_ids_balance("Shard Ticket");
        shard_tickets.sort();

        assert_eq!(shard_tickets, vec![NonFungibleLocalId::from(1), NonFungibleLocalId::from(2), NonFungibleLocalId::from(3)]);

        for i in 1..4 {
            let ticket_data: ShardTicket = test_engine.get_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(i));
            assert_eq!(ticket_data, ShardTicket{shard_type: None})
        }
    }

    #[test]
    fn test_bond_xrd_fails() {
        let mut test_engine = instantiate();

        test_engine.call_method("bond", env_args!(Environment::FungibleBucket("xrd", dec!("3.23")))).assert_failed_with("Incorrect resource address");
    }

    #[test]
    fn test_set_ticket_data() {
        let mut test_engine = instantiate();

        test_engine.call_method("bond", env_args!(Environment::FungibleBucket("shard", dec!("3.23"))));

        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(1), "shard_type", env_args!(Some(ShardType::Blue)), "admin badge").expect_commit_success();

        let ticket_data: ShardTicket = test_engine.get_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(1));
        assert_eq!(ticket_data, ShardTicket{shard_type: Some(ShardType::Blue)});
    }

    #[test]
    fn test_random_cannot_set() {
        let mut test_engine = instantiate();

        test_engine.call_method("bond", env_args!(Environment::FungibleBucket("shard", dec!("3.23"))));

        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(1), "shard_type", env_args!(Some(ShardType::Blue)), "xrd").assert_failed_with("");
    }


    #[test]
    fn test_all_swap_combinations() {
        let mut test_engine = instantiate();

        test_engine.call_method("bond", env_args!(Environment::FungibleBucket("shard", dec!("6"))));

        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(1), "shard_type", env_args!(Some(ShardType::Clear)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(2), "shard_type", env_args!(Some(ShardType::Yellow)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(3), "shard_type", env_args!(Some(ShardType::Orange)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(4), "shard_type", env_args!(Some(ShardType::Blue)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(5), "shard_type", env_args!(Some(ShardType::Scrypto)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(6), "shard_type", env_args!(Some(ShardType::Radix)), "admin badge").expect_commit_success();

        let ids= vec![NonFungibleLocalId::from(1), NonFungibleLocalId::from(2), NonFungibleLocalId::from(3), NonFungibleLocalId::from(4), NonFungibleLocalId::from(5), NonFungibleLocalId::from(6)];

        test_engine.call_method("swap_tickets", env_args!(Environment::NonFungibleBucket("Shard Ticket", ids.clone()))).expect_commit_success();

        let nft_ticket_owned = test_engine.current_ids_balance("Shard Ticket");
        let mut nft_owned = test_engine.current_ids_balance("Shard NFT");
        nft_owned.sort();
        test_engine.current_balance("Shard NFT");

        assert!(nft_ticket_owned.is_empty());
        assert_eq!(nft_owned, ids);
        for nft in nft_owned {
            let data: ShardNFT = test_engine.get_non_fungible_data("Shard NFT", nft.clone());
            match nft {
                NonFungibleLocalId::Integer(id) => {
                    match id.value(){
                        1 => assert_eq!(data.shard_type, ShardType::Clear),
                        2 => assert_eq!(data.shard_type, ShardType::Yellow),
                        3 => assert_eq!(data.shard_type, ShardType::Orange),
                        4 => assert_eq!(data.shard_type, ShardType::Blue),
                        5 => assert_eq!(data.shard_type, ShardType::Scrypto),
                        6 => {assert_eq!(data.shard_type, ShardType::Radix)}
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
        }
    }

    #[test]
    fn test_all_swap_combination_and_destroy() {
        let mut test_engine = instantiate();

        test_engine.call_method("bond", env_args!(Environment::FungibleBucket("shard", dec!("6"))));

        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(1), "shard_type", env_args!(Some(ShardType::Clear)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(2), "shard_type", env_args!(Some(ShardType::Yellow)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(3), "shard_type", env_args!(Some(ShardType::Orange)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(4), "shard_type", env_args!(Some(ShardType::Blue)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(5), "shard_type", env_args!(Some(ShardType::Scrypto)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(6), "shard_type", env_args!(Some(ShardType::Radix)), "admin badge").expect_commit_success();

        let ids= vec![NonFungibleLocalId::from(1), NonFungibleLocalId::from(2), NonFungibleLocalId::from(3), NonFungibleLocalId::from(4), NonFungibleLocalId::from(5), NonFungibleLocalId::from(6)];
        test_engine.call_method("swap_tickets", env_args!(Environment::NonFungibleBucket("Shard Ticket", ids.clone()))).expect_commit_success();

        test_engine.call_method("destroy", env_args!(Environment::NonFungibleBucket("Shard NFT", ids))).assert_failed_with("There is a 4 hour delay between minting and rerolling");
    }

    #[test]
    fn test_all_swap_combination_and_destroy_fails_reroll() {
        let mut test_engine = instantiate();

        test_engine.call_method("bond", env_args!(Environment::FungibleBucket("shard", dec!("6"))));

        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(1), "shard_type", env_args!(Some(ShardType::Clear)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(2), "shard_type", env_args!(Some(ShardType::Yellow)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(3), "shard_type", env_args!(Some(ShardType::Orange)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(4), "shard_type", env_args!(Some(ShardType::Blue)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(5), "shard_type", env_args!(Some(ShardType::Scrypto)), "admin badge").expect_commit_success();
        test_engine.update_non_fungible_data("Shard Ticket", NonFungibleLocalId::from(6), "shard_type", env_args!(Some(ShardType::Radix)), "admin badge").expect_commit_success();

        let ids= vec![NonFungibleLocalId::from(1), NonFungibleLocalId::from(2), NonFungibleLocalId::from(3), NonFungibleLocalId::from(4), NonFungibleLocalId::from(5), NonFungibleLocalId::from(6)];
        test_engine.call_method("swap_tickets", env_args!(Environment::NonFungibleBucket("Shard Ticket", ids.clone()))).expect_commit_success();

        // 4h = 3600*4*1000 ms
        test_engine.advance_time(3600*4*1000);
        test_engine.call_method("destroy", env_args!(Environment::NonFungibleBucket("Shard NFT", ids))).expect_commit_success();


        assert_eq!(test_engine.current_balance( "Shard"), dec!(1000));
        assert_eq!(test_engine.current_balance( "Shard Ticket"), dec!(0));
        assert_eq!(test_engine.current_balance( "Shard NFT"), dec!(0));

    }
}
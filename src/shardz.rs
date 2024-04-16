use scrypto::prelude::*;

#[derive(ScryptoSbor, ManifestSbor, PartialEq, Debug, Clone)]
pub enum ShardType {
    Clear,
    Yellow,
    Orange,
    Blue,
    Scrypto,
    Radix
}

#[derive(NonFungibleData, ScryptoSbor, Debug)]
pub struct ShardNFT {
    name: String,
    key_image_url: Url,
    pub shard_type: ShardType,
    fungible_address: ResourceAddress,
    mint_time: Instant,
}

#[derive(NonFungibleData, ScryptoSbor, PartialEq, Debug)]
pub struct ShardTicket {
    #[mutable]
    pub shard_type: Option<ShardType>
}

impl ShardType {
    fn url(&self) -> Url {
        match self {
            ShardType::Clear => { Url::of("https://bafybeigucelugs5ej3uak3jmwg5tqlotlk2izaezqvvtcdaa2xbnpchdea.ipfs.dweb.link/") }
            ShardType::Yellow => { Url::of("https://bafybeif2hgh76mc4ak4wqqqd5mm36mrboxirlb777cm5gyjpbefu5j75qm.ipfs.dweb.link/") }
            ShardType::Orange => { Url::of("https://bafybeib7lcjwxchh5qv7lwtbizzd2wj5khjrexir6rx25fsqrlspxajk3m.ipfs.dweb.link/") }
            ShardType::Blue => { Url::of("https://bafybeih3ctg2msgjdfcp6jfioyxbcsxkvyw3yxhsz36m4zmogieio3riau.ipfs.dweb.link/") }
            ShardType::Scrypto => { Url::of("https://bafybeihf7lg7mokz3567xhm6leqo6oqte3k3nglg5jmwm67s435bv2zfma.ipfs.dweb.link/") }
            ShardType::Radix => { Url::of("https://bafybeidyubnqsioriwika3hlyrpzu43zc6gipplftgpcjjbghu5xls6zma.ipfs.dweb.link/") }
        }
    }

    fn name(&self) -> String {
        match self {
            ShardType::Clear => { "Clear Shard".to_string() }
            ShardType::Yellow => { "Yellow Shard".to_string() }
            ShardType::Orange => { "Orange Shard".to_string() }
            ShardType::Blue => { "Blue Shard".to_string() }
            ShardType::Scrypto => { "Scrypto Shard".to_string() }
            ShardType::Radix => { "Radix Shard".to_string() }
        }
    }
}

#[blueprint]
#[types(Vault, NonFungibleLocalId, NonFungibleGlobalId)]
mod rrc404 {

    const SHARDZ_BADGE: ResourceAddress = ResourceAddress::new_or_panic([93, 13, 136, 63, 125, 136, 48, 208, 196, 7, 181, 57, 241, 8, 5, 101, 178, 118, 134, 152, 233, 223, 249, 160, 210, 45, 49, 38, 26, 28]);

    const SHARDZ_DESCRIPTION: &str = "Shardz is a revolutionary NFT mini game built on the Radix ledger. 1000 tokens can be shattered and bonded in an attempt to find the rarest shards.";

    struct Shardz {
        shardz_fungible: ResourceManager,
        shardz_nft: ResourceManager,
        shardz_ticket: ResourceManager,
        nft_counter: u64,
        ticket_counter: u64
    }

    impl Shardz {

        pub fn instantiate_shardz(dapp_definition: ComponentAddress) -> (Global<Shardz>, FungibleBucket) {

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(<Shardz>::blueprint_id());
            
            let shardz_fungible = ResourceBuilder::new_fungible(OwnerRole::Fixed(
                rule!(require(global_caller(component_address)))))
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata(metadata! {
                    init {
                        "name" => "Shard", locked;
                        "symbol" => "SHARD", locked;
                        "description" => SHARDZ_DESCRIPTION, locked;
                        "icon_url" => Url::of("https://i.ibb.co/23S8X1B/shard-icon.jpg"), updatable;
                    }
                })
                .mint_roles(mint_roles!{
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all); 
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .mint_initial_supply(dec!(1000));
            
            let shardz_nft = ResourceBuilder::new_integer_non_fungible::<ShardNFT>(OwnerRole::Fixed(
                rule!(require(global_caller(component_address)))))
                .metadata(metadata!(
                    init {
                        "name" => "Shard NFT", updatable;
                        "description" => SHARDZ_DESCRIPTION, locked;
                        "icon_url" => Url::of("https://i.ibb.co/23S8X1B/shard-icon.jpg"), updatable;
                    }
                ))
                .mint_roles(mint_roles!{
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all); 
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                    non_fungible_data_updater => rule!(require(global_caller(component_address)));
                    non_fungible_data_updater_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            let shardz_ticket = ResourceBuilder::new_integer_non_fungible::<ShardTicket>(OwnerRole::Fixed(
                rule!(require(global_caller(component_address)))))
                .metadata(metadata!(
                    init {
                        "name" => "Shard Ticket", updatable;
                        "description" => "Can be traded for a shard NFT", locked;
                        "icon_url" => Url::of("https://bafybeifbuba5i7qxxroxnlvgv34iesddvoiticlmy67oyhwyuvoineffke.ipfs.dweb.link/"), updatable;
                    }
                ))
                .mint_roles(mint_roles!{
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                    non_fungible_data_updater => rule!(require(SHARDZ_BADGE));
                    non_fungible_data_updater_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            let component = Self {
                shardz_fungible: shardz_fungible.resource_manager(),
                shardz_nft,
                shardz_ticket,
                nft_counter: 1,
                ticket_counter: 1
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(SHARDZ_BADGE))))
            .with_address(address_reservation)
            .enable_component_royalties(component_royalties! {
                init {
                    bond => Xrd(1.into()), updatable;
                    destroy => Xrd(1.into()), updatable;
                    swap_tickets => Free, updatable;
                }
            }).metadata(metadata!(roles {
            metadata_setter => rule!(deny_all);
            metadata_setter_updater => rule!(deny_all);
            metadata_locker => rule!(deny_all);
            metadata_locker_updater => rule!(deny_all);
            },
            init {
                    "dapp_definition" => GlobalAddress::from(dapp_definition), updatable;
                    "name" => "Shardz", updatable;
                    "description" => SHARDZ_DESCRIPTION, updatable;
                }))
            .globalize();

            (component, shardz_fungible)
        }

        pub fn bond(&mut self, mut deposit: Bucket) -> (Bucket, Bucket) {
            assert_eq!(deposit.resource_address(), self.shardz_fungible.address(), "Incorrect resource address");
        
            let floor_amount = deposit.amount().checked_floor().unwrap();
            let deposit_amount = floor_amount.to_string().parse::<u64>().unwrap();
            let mut ticket_bucket: Bucket = Bucket::new(self.shardz_ticket.address());
        
            for _ in 0..deposit_amount {
                let nft_id = NonFungibleLocalId::from(self.ticket_counter);
                
                let data = ShardTicket {
                    shard_type: None,
                };
        
                ticket_bucket.put(self.shardz_ticket.mint_non_fungible(&nft_id, data));
                self.ticket_counter += 1;
            }
        
            deposit.take(floor_amount).burn();

        
            (ticket_bucket, deposit)
        }

        pub fn swap_tickets(&mut self, ticket_bucket: Bucket) -> Bucket {
            assert_eq!(ticket_bucket.resource_address(), self.shardz_ticket.address(), "Incorrect resource address");
            let mut nft_bucket: Bucket = Bucket::new(self.shardz_nft.address());

            for nft_ticket in ticket_bucket.as_non_fungible().non_fungibles(){
                let ticket: ShardTicket = nft_ticket.data();
                if let Some(shard_type) = ticket.shard_type {
                    let nft_id = NonFungibleLocalId::from(self.nft_counter);

                    let data = ShardNFT {
                        name: shard_type.name(),
                        key_image_url: shard_type.url(),
                        shard_type,
                        fungible_address: self.shardz_fungible.address(),
                        mint_time: Clock::current_time_rounded_to_minutes(),
                    };

                    nft_bucket.put(self.shardz_nft.mint_non_fungible(&nft_id, data));
                    self.nft_counter+=1;
                }
                else{
                    panic!("Some ticket were not drawn")
                }
            }
            ticket_bucket.burn();

            nft_bucket
        }


        pub fn destroy(&mut self, nft_bucket: Bucket) -> Bucket{
            
            // Assert resource address matches the resource address of the vault
            assert_eq!(nft_bucket.resource_address(), self.shardz_nft.address(), "Incorrect resource address");

            for nft_id in nft_bucket.as_non_fungible().non_fungible_local_ids() {
                
                let data = self.shardz_nft.get_non_fungible_data::<ShardNFT>(&nft_id);

                // Check that each nft is past a 4 hour cooldown period for rerolling
                let last_roll = data.mint_time;
                let last_roll_utc = UtcDateTime::try_from(last_roll).unwrap();
                let next_roll_utc= last_roll_utc.add_hours(4).unwrap();
                let next_roll = Instant::from(next_roll_utc);

                assert!(Clock::current_time_is_at_or_after(next_roll, TimePrecision::Minute),
                    "There is a 4 hour delay between minting and rerolling"
                );

            }

            let fungible_bucket = self.shardz_fungible.mint(nft_bucket.amount());
            nft_bucket.burn();

            fungible_bucket
        }
    }
}
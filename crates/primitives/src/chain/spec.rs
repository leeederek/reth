use crate::{
    BlockNumber, Chain, ForkDiscriminant, ForkFilter, ForkHash, ForkId, ForkKind, Genesis,
    GenesisAccount, Hardfork, Header, H160, H256, U256,
};
use ethers_core::utils::Genesis as EthersGenesis;
use hex_literal::hex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// The Etereum mainnet spec
pub static MAINNET: Lazy<ChainSpec> = Lazy::new(|| ChainSpec {
    chain: Chain::mainnet(),
    genesis: serde_json::from_str(include_str!("../../res/genesis/mainnet.json"))
        .expect("Can't deserialize Mainnet genesis json"),
    genesis_hash: H256(hex!("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3")),
    hardforks: BTreeMap::from([
        (Hardfork::Frontier, ForkKind::Block(0)),
        (Hardfork::Homestead, ForkKind::Block(1150000)),
        (Hardfork::Dao, ForkKind::Block(1920000)),
        (Hardfork::Tangerine, ForkKind::Block(2463000)),
        (Hardfork::SpuriousDragon, ForkKind::Block(2675000)),
        (Hardfork::Byzantium, ForkKind::Block(4370000)),
        (Hardfork::Constantinople, ForkKind::Block(7280000)),
        (Hardfork::Petersburg, ForkKind::Block(7280000)),
        (Hardfork::Istanbul, ForkKind::Block(9069000)),
        (Hardfork::Muirglacier, ForkKind::Block(9200000)),
        (Hardfork::Berlin, ForkKind::Block(12244000)),
        (Hardfork::London, ForkKind::Block(12965000)),
        (Hardfork::ArrowGlacier, ForkKind::Block(13773000)),
        (Hardfork::GrayGlacier, ForkKind::Block(15050000)),
        (Hardfork::Paris, ForkKind::TTD(Some(15537394))),
    ]),
    dao_fork_support: true,
    paris_ttd: Some(U256::from(58_750_000_000_000_000_000_000_u128)),
});

/// The Goerli spec
pub static GOERLI: Lazy<ChainSpec> = Lazy::new(|| ChainSpec {
    chain: Chain::goerli(),
    genesis: serde_json::from_str(include_str!("../../res/genesis/goerli.json"))
        .expect("Can't deserialize Goerli genesis json"),
    genesis_hash: H256(hex!("bf7e331f7f7c1dd2e05159666b3bf8bc7a8a3a9eb1d518969eab529dd9b88c1a")),
    hardforks: BTreeMap::from([
        (Hardfork::Frontier, ForkKind::Block(0)),
        (Hardfork::Istanbul, ForkKind::Block(1561651)),
        (Hardfork::Berlin, ForkKind::Block(4460644)),
        (Hardfork::London, ForkKind::Block(5062605)),
        (Hardfork::Paris, ForkKind::TTD(Some(7382818))),
    ]),
    dao_fork_support: true,
    paris_ttd: Some(U256::from(10_790_000)),
});

/// The Sepolia spec
pub static SEPOLIA: Lazy<ChainSpec> = Lazy::new(|| ChainSpec {
    chain: Chain::sepolia(),
    genesis: serde_json::from_str(include_str!("../../res/genesis/sepolia.json"))
        .expect("Can't deserialize Sepolia genesis json"),
    genesis_hash: H256(hex!("25a5cc106eea7138acab33231d7160d69cb777ee0c2c553fcddf5138993e6dd9")),
    hardforks: BTreeMap::from([
        (Hardfork::Frontier, ForkKind::Block(0)),
        (Hardfork::Homestead, ForkKind::Block(0)),
        (Hardfork::Dao, ForkKind::Block(0)),
        (Hardfork::Tangerine, ForkKind::Block(0)),
        (Hardfork::SpuriousDragon, ForkKind::Block(0)),
        (Hardfork::Byzantium, ForkKind::Block(0)),
        (Hardfork::Constantinople, ForkKind::Block(0)),
        (Hardfork::Petersburg, ForkKind::Block(0)),
        (Hardfork::Istanbul, ForkKind::Block(0)),
        (Hardfork::Muirglacier, ForkKind::Block(0)),
        (Hardfork::Berlin, ForkKind::Block(0)),
        (Hardfork::London, ForkKind::Block(0)),
        (Hardfork::Paris, ForkKind::Block(1735371)),
    ]),
    dao_fork_support: true,
    paris_ttd: Some(U256::from(17_000_000_000_000_000_u64)),
});

/// The Ethereum chain spec
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainSpec {
    /// The chain id
    pub chain: Chain,

    /// The genesis block information
    pub genesis: Genesis,

    /// The genesis block hash
    pub genesis_hash: H256,

    /// The active hard forks and their block numbers
    pub hardforks: BTreeMap<Hardfork, ForkKind>,

    /// Whether or not the DAO fork is supported
    pub dao_fork_support: bool,

    /// The merge terminal total difficulty
    pub paris_ttd: Option<U256>,
}

impl ChainSpec {
    /// Returns the chain id
    pub fn chain(&self) -> Chain {
        self.chain
    }

    /// Return genesis block
    pub fn genesis(&self) -> &Genesis {
        &self.genesis
    }

    /// Returns the chain genesis hash
    pub fn genesis_hash(&self) -> H256 {
        self.genesis_hash
    }

    /// Returns the supported hardforks and their [ForkKind]
    pub fn hardforks(&self) -> &BTreeMap<Hardfork, ForkKind> {
        &self.hardforks
    }

    /// Get the first block number of the given hardfork. This method returns `None` for
    /// timestamp-based forks and if the merge netsplit block is not known (when the given fork
    /// is TTD-based)
    pub fn fork_block(&self, fork: Hardfork) -> Option<BlockNumber> {
        self.hardforks.get(&fork).and_then(|kind| match kind {
            ForkKind::Block(block_number) => Some(*block_number),
            ForkKind::TTD(block_number) => *block_number,
            _ => None,
        })
    }

    /// Get the first block number/timestamp of the hardfork.
    pub fn fork_kind(&self, fork: Hardfork) -> Option<ForkKind> {
        self.hardforks.get(&fork).copied()
    }

    /// Returns `true` if the given fork is active on the given should update docs here to reflect
    /// that this returns true if the fork is active on the given block / ttd / timestamp contained
    /// in the [ForkDiscriminant]
    pub fn fork_active(&self, fork: Hardfork, discriminant: ForkDiscriminant) -> bool {
        match self.hardforks.get(&fork) {
            Some(kind) => match kind {
                ForkKind::Block(block_number) => *block_number <= discriminant.block_number,
                ForkKind::TTD(block_number) => {
                    self.paris_ttd <= Some(discriminant.total_difficulty) ||
                        *block_number <= Some(discriminant.block_number)
                }
                ForkKind::Time(timestamp) => *timestamp <= discriminant.timestamp,
            },
            None => false,
        }
    }

    /// Returns `true` if the DAO fork is supported
    pub fn dao_fork_support(&self) -> bool {
        self.dao_fork_support
    }

    /// The merge terminal total difficulty
    pub fn terminal_total_difficulty(&self) -> Option<U256> {
        self.paris_ttd
    }

    /// Get an iterator of all harforks with theirs respectives [ForkKind]
    pub fn forks_iter(&self) -> impl Iterator<Item = (Hardfork, ForkKind)> + '_ {
        self.hardforks.iter().map(|(f, b)| (*f, *b))
    }

    /// Creates a [`ForkFilter`](crate::ForkFilter) for the given [ForkDiscriminant].
    pub fn fork_filter(&self, discriminant: ForkDiscriminant) -> ForkFilter {
        let future_forks = self
            .forks_iter()
            .filter(|(f, _)| !self.fork_active(*f, discriminant))
            .filter_map(|(_, k)| match k {
                ForkKind::Block(block_number) => Some(block_number),
                ForkKind::TTD(_) => None,
                ForkKind::Time(timestamp) => Some(timestamp),
            })
            .collect::<Vec<_>>();

        ForkFilter::new(discriminant.block_number, self.genesis_hash(), future_forks)
    }

    /// Compute the forkid for the given [ForkDiscriminant]
    pub fn fork_id(&self, discriminant: ForkDiscriminant) -> ForkId {
        let mut curr_forkhash = ForkHash::from(self.genesis_hash());
        let mut forks =
            self.forks_iter().filter(|(_, k)| !k.is_active_at_genesis()).collect::<Vec<_>>();

        forks.dedup_by(|a, b| a.1 == b.1);

        for (_, kind) in forks {
            match kind {
                ForkKind::Block(b) => {
                    if discriminant.block_number >= b {
                        curr_forkhash += b;
                    } else {
                        return ForkId { hash: curr_forkhash, next: b }
                    }
                }
                ForkKind::Time(t) => {
                    if discriminant.timestamp >= t {
                        curr_forkhash += t;
                    } else {
                        return ForkId { hash: curr_forkhash, next: t }
                    }
                }
                _ => {}
            }
        }
        ForkId { hash: curr_forkhash, next: 0 }
    }

    /// Returns a [ChainSpecBuilder] to help build custom specs
    pub fn builder() -> ChainSpecBuilder {
        ChainSpecBuilder::default()
    }
}

impl From<EthersGenesis> for ChainSpec {
    fn from(genesis: EthersGenesis) -> Self {
        let alloc = genesis
            .alloc
            .iter()
            .map(|(addr, account)| (addr.0.into(), account.clone().into()))
            .collect::<HashMap<H160, GenesisAccount>>();

        let genesis_block = Genesis {
            nonce: genesis.nonce.as_u64(),
            timestamp: genesis.timestamp.as_u64(),
            gas_limit: genesis.gas_limit.as_u64(),
            difficulty: genesis.difficulty.into(),
            mix_hash: genesis.mix_hash.0.into(),
            coinbase: genesis.coinbase.0.into(),
            extra_data: genesis.extra_data.0.into(),
            alloc,
        };

        let genesis_hash = Header::from(genesis_block.clone()).seal().hash();
        let paris_ttd = genesis.config.terminal_total_difficulty.map(|ttd| ttd.into());
        let mut hardfork_opts = vec![
            (Hardfork::Homestead, genesis.config.homestead_block),
            (Hardfork::Dao, genesis.config.dao_fork_block),
            (Hardfork::Tangerine, genesis.config.eip150_block),
            (Hardfork::SpuriousDragon, genesis.config.eip155_block),
            (Hardfork::Byzantium, genesis.config.byzantium_block),
            (Hardfork::Constantinople, genesis.config.constantinople_block),
            (Hardfork::Petersburg, genesis.config.petersburg_block),
            (Hardfork::Istanbul, genesis.config.istanbul_block),
            (Hardfork::Muirglacier, genesis.config.muir_glacier_block),
            (Hardfork::Berlin, genesis.config.berlin_block),
            (Hardfork::London, genesis.config.london_block),
            (Hardfork::ArrowGlacier, genesis.config.arrow_glacier_block),
            (Hardfork::GrayGlacier, genesis.config.gray_glacier_block),
        ];

        // Paris block is not used to fork, and is not used in genesis.json
        // except in Sepolia
        if genesis.config.chain_id == Chain::sepolia().id() {
            hardfork_opts.push((Hardfork::Paris, genesis.config.merge_netsplit_block))
        }

        let configured_hardforks = hardfork_opts
            .iter()
            .filter_map(|(hardfork, opt)| opt.map(|block| (*hardfork, ForkKind::Block(block))))
            .collect::<BTreeMap<_, _>>();

        Self {
            chain: genesis.config.chain_id.into(),
            dao_fork_support: genesis.config.dao_fork_support,
            genesis: genesis_block,
            hardforks: configured_hardforks,
            genesis_hash,
            paris_ttd,
        }
    }
}

/// A helper to build custom chain specs
#[derive(Debug, Default)]
pub struct ChainSpecBuilder {
    chain: Option<Chain>,
    genesis: Option<Genesis>,
    genesis_hash: Option<H256>,
    hardforks: BTreeMap<Hardfork, ForkKind>,
    dao_fork_support: bool,
    paris_ttd: Option<U256>,
}

impl ChainSpecBuilder {
    /// Returns a [ChainSpec] builder initialized with Ethereum mainnet config
    pub fn mainnet() -> Self {
        Self {
            chain: Some(MAINNET.chain),
            genesis: Some(MAINNET.genesis.clone()),
            genesis_hash: Some(MAINNET.genesis_hash),
            hardforks: MAINNET.hardforks.clone(),
            dao_fork_support: MAINNET.dao_fork_support,
            paris_ttd: MAINNET.paris_ttd,
        }
    }

    /// Sets the chain id
    pub fn chain(mut self, chain: Chain) -> Self {
        self.chain = Some(chain);
        self
    }

    /// Sets the genesis
    pub fn genesis(mut self, genesis: Genesis) -> Self {
        self.genesis = Some(genesis);
        self
    }

    /// Sets the genesis hash
    pub fn genesis_hash(mut self, genesis_hash: H256) -> Self {
        self.genesis_hash = Some(genesis_hash);
        self
    }

    /// Remove all [Hardfork]s from the spec. Useful when the builder has been created from a
    /// existing chain spec.
    pub fn clear_forks(mut self) -> Self {
        self.hardforks.clear();
        self
    }

    /// Insert the given fork at the given [ForkKind]
    pub fn with_fork(mut self, fork: Hardfork, kind: ForkKind) -> Self {
        self.hardforks.insert(fork, kind);
        self
    }

    /// Enables Frontier
    pub fn frontier_activated(mut self) -> Self {
        self.hardforks.insert(Hardfork::Frontier, ForkKind::Block(0));
        self
    }

    /// Enables Homestead
    pub fn homestead_activated(mut self) -> Self {
        self = self.frontier_activated();
        self.hardforks.insert(Hardfork::Homestead, ForkKind::Block(0));
        self
    }

    /// Enables Tangerine
    pub fn tangerine_whistle_activated(mut self) -> Self {
        self = self.homestead_activated();
        self.hardforks.insert(Hardfork::Tangerine, ForkKind::Block(0));
        self
    }

    /// Enables SpuriousDragon
    pub fn spurious_dragon_activated(mut self) -> Self {
        self = self.tangerine_whistle_activated();
        self.hardforks.insert(Hardfork::SpuriousDragon, ForkKind::Block(0));
        self
    }

    /// Enables Byzantium
    pub fn byzantium_activated(mut self) -> Self {
        self = self.spurious_dragon_activated();
        self.hardforks.insert(Hardfork::Byzantium, ForkKind::Block(0));
        self
    }

    /// Enables Petersburg
    pub fn petersburg_activated(mut self) -> Self {
        self = self.byzantium_activated();
        self.hardforks.insert(Hardfork::Petersburg, ForkKind::Block(0));
        self
    }

    /// Enables Istanbul
    pub fn istanbul_activated(mut self) -> Self {
        self = self.petersburg_activated();
        self.hardforks.insert(Hardfork::Istanbul, ForkKind::Block(0));
        self
    }

    /// Enables Berlin
    pub fn berlin_activated(mut self) -> Self {
        self = self.istanbul_activated();
        self.hardforks.insert(Hardfork::Berlin, ForkKind::Block(0));
        self
    }

    /// Enables London
    pub fn london_activated(mut self) -> Self {
        self = self.berlin_activated();
        self.hardforks.insert(Hardfork::London, ForkKind::Block(0));
        self
    }

    /// Enables Paris
    pub fn paris_activated(mut self) -> Self {
        self = self.berlin_activated();
        self.hardforks.insert(Hardfork::Paris, ForkKind::TTD(Some(0)));
        self
    }

    /// Enables Shangai
    pub fn shangai_activated(mut self) -> Self {
        self = self.paris_activated();
        self.hardforks.insert(Hardfork::Shanghai, ForkKind::Time(0));
        self
    }

    /// Sets the DAO fork as supported
    pub fn dao_fork_supported(mut self) -> Self {
        self.dao_fork_support = true;
        self
    }

    /// Build a [ChainSpec]
    pub fn build(self) -> ChainSpec {
        ChainSpec {
            chain: self.chain.expect("The chain is required"),
            genesis: self.genesis.expect("The genesis is required"),
            genesis_hash: self.genesis_hash.expect("The genesis hash is required"),
            hardforks: self.hardforks,
            dao_fork_support: self.dao_fork_support,
            paris_ttd: self.paris_ttd,
        }
    }
}

impl From<&ChainSpec> for ChainSpecBuilder {
    fn from(value: &ChainSpec) -> Self {
        Self {
            chain: Some(value.chain),
            genesis: Some(value.genesis.clone()),
            genesis_hash: Some(value.genesis_hash),
            hardforks: value.hardforks.clone(),
            dao_fork_support: value.dao_fork_support,
            paris_ttd: value.paris_ttd,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Chain, ChainSpec, ForkDiscriminant, ForkHash, ForkKind, Genesis, Hardfork, Header, GOERLI,
        MAINNET, SEPOLIA,
    };

    #[test]
    fn test_empty_forkid() {
        // tests that we skip any forks in block 0, that's the genesis ruleset
        let empty_genesis = Genesis::default();
        let empty_header: Header = empty_genesis.clone().into();
        let empty_sealed = empty_header.seal();
        let spec = ChainSpec::builder()
            .chain(Chain::mainnet())
            .genesis(empty_genesis)
            .genesis_hash(empty_sealed.hash())
            .clear_forks()
            .with_fork(Hardfork::Frontier, ForkKind::Block(0))
            .with_fork(Hardfork::Homestead, ForkKind::Block(0))
            .with_fork(Hardfork::Tangerine, ForkKind::Block(0))
            .with_fork(Hardfork::SpuriousDragon, ForkKind::Block(0))
            .with_fork(Hardfork::Byzantium, ForkKind::Block(0))
            .with_fork(Hardfork::Constantinople, ForkKind::Block(0))
            .with_fork(Hardfork::Istanbul, ForkKind::Block(0))
            .with_fork(Hardfork::Muirglacier, ForkKind::Block(0))
            .with_fork(Hardfork::Berlin, ForkKind::Block(0))
            .with_fork(Hardfork::London, ForkKind::Block(0))
            .with_fork(Hardfork::ArrowGlacier, ForkKind::Block(0))
            .with_fork(Hardfork::GrayGlacier, ForkKind::Block(0))
            .build();

        // test at block one - all forks should be active
        let res_forkid = spec.fork_id(1_u64.into());
        let expected_forkhash = ForkHash::from(spec.genesis_hash());

        // if blocks get activated at genesis then they should not be accumulated into the forkhash
        assert_eq!(res_forkid.hash, expected_forkhash);
    }

    #[test]
    fn test_duplicate_fork_blocks() {
        // forks activated at the same block should be deduplicated
        let empty_genesis = Genesis::default();
        let empty_header: Header = empty_genesis.clone().into();
        let empty_sealed = empty_header.seal();
        let unique_spec = ChainSpec::builder()
            .chain(Chain::mainnet())
            .genesis(empty_genesis.clone())
            .genesis_hash(empty_sealed.hash())
            .with_fork(Hardfork::Frontier, ForkKind::Block(0))
            .with_fork(Hardfork::Homestead, ForkKind::Block(1))
            .build();

        let duplicate_spec = ChainSpec::builder()
            .chain(Chain::mainnet())
            .genesis(empty_genesis)
            .genesis_hash(empty_sealed.hash())
            .with_fork(Hardfork::Frontier, ForkKind::Block(0))
            .with_fork(Hardfork::Homestead, ForkKind::Block(1))
            .with_fork(Hardfork::Tangerine, ForkKind::Block(1))
            .build();

        assert_eq!(unique_spec.fork_id(2.into()), duplicate_spec.fork_id(2.into()));
    }

    // these tests check that the forkid computation is accurate
    #[test]
    fn test_mainnet_forkids() {
        let frontier_forkid = MAINNET.fork_id(0.into());
        assert_eq!([0xfc, 0x64, 0xec, 0x04], frontier_forkid.hash.0);
        assert_eq!(1150000, frontier_forkid.next);

        let homestead_forkid = MAINNET.fork_id(1150000.into());
        assert_eq!([0x97, 0xc2, 0xc3, 0x4c], homestead_forkid.hash.0);
        assert_eq!(1920000, homestead_forkid.next);

        let dao_forkid = MAINNET.fork_id(1920000.into());
        assert_eq!([0x91, 0xd1, 0xf9, 0x48], dao_forkid.hash.0);
        assert_eq!(2463000, dao_forkid.next);

        let tangerine_forkid = MAINNET.fork_id(2463000.into());
        assert_eq!([0x7a, 0x64, 0xda, 0x13], tangerine_forkid.hash.0);
        assert_eq!(2675000, tangerine_forkid.next);

        let spurious_forkid = MAINNET.fork_id(2675000.into());
        assert_eq!([0x3e, 0xdd, 0x5b, 0x10], spurious_forkid.hash.0);
        assert_eq!(4370000, spurious_forkid.next);

        let byzantium_forkid = MAINNET.fork_id(4370000.into());
        assert_eq!([0xa0, 0x0b, 0xc3, 0x24], byzantium_forkid.hash.0);
        assert_eq!(7280000, byzantium_forkid.next);

        let constantinople_forkid = MAINNET.fork_id(7280000.into());
        assert_eq!([0x66, 0x8d, 0xb0, 0xaf], constantinople_forkid.hash.0);
        assert_eq!(9069000, constantinople_forkid.next);

        let istanbul_forkid = MAINNET.fork_id(9069000.into());
        assert_eq!([0x87, 0x9d, 0x6e, 0x30], istanbul_forkid.hash.0);
        assert_eq!(9200000, istanbul_forkid.next);

        let muir_glacier_forkid = MAINNET.fork_id(9200000.into());
        assert_eq!([0xe0, 0x29, 0xe9, 0x91], muir_glacier_forkid.hash.0);
        assert_eq!(12244000, muir_glacier_forkid.next);

        let berlin_forkid = MAINNET.fork_id(12244000.into());
        assert_eq!([0x0e, 0xb4, 0x40, 0xf6], berlin_forkid.hash.0);
        assert_eq!(12965000, berlin_forkid.next);

        let london_forkid = MAINNET.fork_id(12965000.into());
        assert_eq!([0xb7, 0x15, 0x07, 0x7d], london_forkid.hash.0);
        assert_eq!(13773000, london_forkid.next);

        let arrow_glacier_forkid = MAINNET.fork_id(13773000.into());
        assert_eq!([0x20, 0xc3, 0x27, 0xfc], arrow_glacier_forkid.hash.0);
        assert_eq!(15050000, arrow_glacier_forkid.next);

        let gray_glacier_forkid = MAINNET.fork_id(15050000.into());
        assert_eq!([0xf0, 0xaf, 0xd0, 0xe3], gray_glacier_forkid.hash.0);
        assert_eq!(0, gray_glacier_forkid.next); // TODO: update post-gray glacier

        let latest_forkid = MAINNET.fork_id(15050000.into());
        assert_eq!(0, latest_forkid.next);
    }

    #[test]
    fn test_goerli_forkids() {
        let frontier_forkid = GOERLI.fork_id(ForkDiscriminant::block(0));
        assert_eq!([0xa3, 0xf5, 0xab, 0x08], frontier_forkid.hash.0);
        assert_eq!(1561651, frontier_forkid.next);

        let istanbul_forkid = GOERLI.fork_id(ForkDiscriminant::block(1561651));
        assert_eq!([0xc2, 0x5e, 0xfa, 0x5c], istanbul_forkid.hash.0);
        assert_eq!(4460644, istanbul_forkid.next);

        let berlin_forkid = GOERLI.fork_id(ForkDiscriminant::block(4460644));
        assert_eq!([0x75, 0x7a, 0x1c, 0x47], berlin_forkid.hash.0);
        assert_eq!(5062605, berlin_forkid.next);

        let london_forkid = GOERLI.fork_id(ForkDiscriminant::block(12965000));
        assert_eq!([0xb8, 0xc6, 0x29, 0x9d], london_forkid.hash.0);
        assert_eq!(0, london_forkid.next);
    }

    #[test]
    fn test_sepolia_forkids() {
        // Test vector is from <https://github.com/ethereum/go-ethereum/blob/59a48e0289b1a7470a8285e665cab12b29117a70/core/forkid/forkid_test.go#L146-L151>
        let mergenetsplit_forkid = SEPOLIA.fork_id(ForkDiscriminant::block(1735371));
        assert_eq!([0xb9, 0x6c, 0xbd, 0x13], mergenetsplit_forkid.hash.0);
        assert_eq!(0, mergenetsplit_forkid.next);
    }
}
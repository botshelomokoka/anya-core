#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use mockall::predicate::*;
    use mockall::*;

    // Mock the fs module
    mock! {
        FS {}
        trait FS {
            fn create_dir_all<P: AsRef<Path>>(path: P) -> std::io::Result<()>;
        }
    }

    #[test]
    fn test_setup_common_environment() {
        let mut mock_fs = MockFS::new();
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/src"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/tests"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/stx"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/dlc"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/lightning"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/bitcoin"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/web5"))
            .times(1)
            .returning(|_| Ok(()));

        let project_setup = ProjectSetup {
            logger: slog::Logger::root(slog::Discard, o!()),
            user_type: UserType::Normal,
            user_data: HashMap::new(),
            project_name: String::from("anya-core"),
            user_management: UserManagement::new().unwrap(),
            node: Node::new(),
            network_discovery: NetworkDiscovery::new(),
            main_system: MainSystem::new(),
            ml_logic: MLLogic::new(),
            stx_support: STXSupport::new().unwrap(),
            dlc_support: DLCSupport::new().unwrap(),
            lightning_support: LightningSupport::new().unwrap(),
            bitcoin_support: BitcoinSupport::new().unwrap(),
            web5_support: Web5Support::new().unwrap(),
            libp2p_support: Libp2pSupport::new().unwrap(),
            unified_network: UnifiedNetworkManager::new().unwrap(),
            cross_chain: CrossChainManager::new().unwrap(),
            cross_network_fl: CrossNetworkFederatedLearning::new().unwrap(),
            interoperability: InteroperabilityProtocol::new().unwrap(),
        };

        assert!(project_setup.setup_common_environment().is_ok());
    }

    #[test]
    fn test_setup_creator_project() {
        let mut mock_fs = MockFS::new();
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/admin_tools"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/stx/contracts"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/dlc/contracts"))
            .times(1)
            .returning(|_| Ok(()));

        let project_setup = ProjectSetup {
            logger: slog::Logger::root(slog::Discard, o!()),
            user_type: UserType::Creator,
            user_data: HashMap::new(),
            project_name: String::from("anya-core"),
            user_management: UserManagement::new().unwrap(),
            node: Node::new(),
            network_discovery: NetworkDiscovery::new(),
            main_system: MainSystem::new(),
            ml_logic: MLLogic::new(),
            stx_support: STXSupport::new().unwrap(),
            dlc_support: DLCSupport::new().unwrap(),
            lightning_support: LightningSupport::new().unwrap(),
            bitcoin_support: BitcoinSupport::new().unwrap(),
            web5_support: Web5Support::new().unwrap(),
            libp2p_support: Libp2pSupport::new().unwrap(),
            unified_network: UnifiedNetworkManager::new().unwrap(),
            cross_chain: CrossChainManager::new().unwrap(),
            cross_network_fl: CrossNetworkFederatedLearning::new().unwrap(),
            interoperability: InteroperabilityProtocol::new().unwrap(),
        };

        assert!(project_setup.setup_creator_project().is_ok());
    }

    #[test]
    fn test_setup_developer_project() {
        let mut mock_fs = MockFS::new();
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/dev_env"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/stx/tests"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/dlc/tests"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/lightning/tests"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/bitcoin/tests"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/web5/tests"))
            .times(1)
            .returning(|_| Ok(()));

        let project_setup = ProjectSetup {
            logger: slog::Logger::root(slog::Discard, o!()),
            user_type: UserType::Developer,
            user_data: HashMap::new(),
            project_name: String::from("anya-core"),
            user_management: UserManagement::new().unwrap(),
            node: Node::new(),
            network_discovery: NetworkDiscovery::new(),
            main_system: MainSystem::new(),
            ml_logic: MLLogic::new(),
            stx_support: STXSupport::new().unwrap(),
            dlc_support: DLCSupport::new().unwrap(),
            lightning_support: LightningSupport::new().unwrap(),
            bitcoin_support: BitcoinSupport::new().unwrap(),
            web5_support: Web5Support::new().unwrap(),
            libp2p_support: Libp2pSupport::new().unwrap(),
            unified_network: UnifiedNetworkManager::new().unwrap(),
            cross_chain: CrossChainManager::new().unwrap(),
            cross_network_fl: CrossNetworkFederatedLearning::new().unwrap(),
            interoperability: InteroperabilityProtocol::new().unwrap(),
        };

        assert!(project_setup.setup_developer_project().is_ok());
    }

    #[test]
    fn test_setup_normal_user_project() {
        let mut mock_fs = MockFS::new();
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/user_interface"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/local_storage"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/web5"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/stx/wallet"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/dlc/wallet"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/lightning/wallet"))
            .times(1)
            .returning(|_| Ok(()));
        mock_fs.expect_create_dir_all()
            .with(eq("anya-core/bitcoin/wallet"))
            .times(1)
            .returning(|_| Ok(()));

        let project_setup = ProjectSetup {
            logger: slog::Logger::root(slog::Discard, o!()),
            user_type: UserType::Normal,
            user_data: HashMap::new(),
            project_name: String::from("anya-core"),
            user_management: UserManagement::new().unwrap(),
            node: Node::new(),
            network_discovery: NetworkDiscovery::new(),
            main_system: MainSystem::new(),
            ml_logic: MLLogic::new(),
            stx_support: STXSupport::new().unwrap(),
            dlc_support: DLCSupport::new().unwrap(),
            lightning_support: LightningSupport::new().unwrap(),
            bitcoin_support: BitcoinSupport::new().unwrap(),
            web5_support: Web5Support::new().unwrap(),
            libp2p_support: Libp2pSupport::new().unwrap(),
            unified_network: UnifiedNetworkManager::new().unwrap(),
            cross_chain: CrossChainManager::new().unwrap(),
            cross_network_fl: CrossNetworkFederatedLearning::new().unwrap(),
            interoperability: InteroperabilityProtocol::new().unwrap(),
        };

        assert!(project_setup.setup_normal_user_project().is_ok());
    }
}
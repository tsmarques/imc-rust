#[cfg(test)]
mod tests {
    use crate::imc::Announce::Announce;
    use crate::imc::AnnounceService::AnnounceService;
    use crate::imc::CacheControl::CacheControl;
    use crate::imc::ClockControl::ClockControl;
    use crate::imc::CpuUsage::CpuUsage;
    use crate::imc::DesiredPath::DesiredPath;
    use crate::imc::EntityActivationState::EntityActivationState;
    use crate::imc::EntityInfo::EntityInfo;
    use crate::imc::EntityList::EntityList;
    use crate::imc::EntityState::EntityState;
    use crate::imc::EstimatedState::EstimatedState;
    use crate::imc::ExtendedRSSI::ExtendedRSSI;
    use crate::imc::Heartbeat::Heartbeat;
    use crate::imc::IridiumMsgRx::IridiumMsgRx;
    use crate::imc::IridiumMsgTx::IridiumMsgTx;
    use crate::imc::IridiumTxStatus::IridiumTxStatus;
    use crate::imc::LblBeacon::LblBeacon;
    use crate::imc::LblRange::LblRange;
    use crate::imc::LogBookControl::LogBookControl;
    use crate::imc::LogBookEntry::LogBookEntry;
    use crate::imc::LoggingControl::LoggingControl;
    use crate::imc::Message::Message;
    use crate::imc::MsgList::MsgList;
    use crate::imc::NavigationUncertainty::NavigationUncertainty;
    use crate::imc::PathControlState::PathControlState;
    use crate::imc::QueryEntityActivationState::QueryEntityActivationState;
    use crate::imc::QueryEntityInfo::QueryEntityInfo;
    use crate::imc::QueryEntityState::QueryEntityState;
    use crate::imc::ReplayControl::ReplayControl;
    use crate::imc::RestartSystem::RestartSystem;
    use crate::imc::SimulatedState::SimulatedState;
    use crate::imc::Sms::Sms;
    use crate::imc::SmsRx::SmsRx;
    use crate::imc::SmsState::SmsState;
    use crate::imc::SmsTx::SmsTx;
    use crate::imc::StorageUsage::StorageUsage;
    use crate::imc::TextMessage::TextMessage;
    use crate::imc::TransportBindings::TransportBindings;
    use crate::imc::VehicleOperationalLimits::VehicleOperationalLimits;
    use crate::imc::RSSI::RSSI;

    #[test]
    fn test_fixed_serialization_size_EntityState() {
        assert_eq!(EntityState::new().fixed_serialization_size(), 2);
    }

    #[test]
    fn test_fixed_serialization_size_QueryEntityState() {
        assert_eq!(QueryEntityState::new().fixed_serialization_size(), 0);
    }

    #[test]
    fn test_fixed_serialization_size_EntityInfo() {
        assert_eq!(EntityInfo::new().fixed_serialization_size(), 5);
    }

    #[test]
    fn test_fixed_serialization_size_QueryEntityInfo() {
        assert_eq!(QueryEntityInfo::new().fixed_serialization_size(), 1);
    }

    #[test]
    fn test_fixed_serialization_size_EntityList() {
        assert_eq!(EntityList::new().fixed_serialization_size(), 1);
    }

    #[test]
    fn test_fixed_serialization_size_CpuUsage() {
        assert_eq!(CpuUsage::new().fixed_serialization_size(), 1);
    }

    #[test]
    fn test_fixed_serialization_size_TransportBindings() {
        assert_eq!(TransportBindings::new().fixed_serialization_size(), 2);
    }

    #[test]
    fn test_fixed_serialization_size_RestartSystem() {
        assert_eq!(RestartSystem::new().fixed_serialization_size(), 0);
    }

    #[test]
    fn test_fixed_serialization_size_EntityActivationState() {
        assert_eq!(EntityActivationState::new().fixed_serialization_size(), 1);
    }

    #[test]
    fn test_fixed_serialization_size_QueryEntityActivationState() {
        assert_eq!(
            QueryEntityActivationState::new().fixed_serialization_size(),
            0
        );
    }

    #[test]
    fn test_fixed_serialization_size_VehicleOperationalLimits() {
        assert_eq!(
            VehicleOperationalLimits::new().fixed_serialization_size(),
            69
        );
    }

    #[test]
    fn test_fixed_serialization_size_MsgList() {
        assert_eq!(MsgList::new().fixed_serialization_size(), 0);
    }

    #[test]
    fn test_fixed_serialization_size_SimulatedState() {
        assert_eq!(SimulatedState::new().fixed_serialization_size(), 80);
    }

    #[test]
    fn test_fixed_serialization_size_StorageUsage() {
        assert_eq!(StorageUsage::new().fixed_serialization_size(), 5);
    }

    #[test]
    fn test_fixed_serialization_size_CacheControl() {
        assert_eq!(CacheControl::new().fixed_serialization_size(), 1);
    }

    #[test]
    fn test_fixed_serialization_size_LoggingControl() {
        assert_eq!(LoggingControl::new().fixed_serialization_size(), 1);
    }

    #[test]
    fn test_fixed_serialization_size_LogBookEntry() {
        assert_eq!(LogBookEntry::new().fixed_serialization_size(), 9);
    }

    #[test]
    fn test_fixed_serialization_size_LogBookControl() {
        assert_eq!(LogBookControl::new().fixed_serialization_size(), 9);
    }

    #[test]
    fn test_fixed_serialization_size_ReplayControl() {
        assert_eq!(ReplayControl::new().fixed_serialization_size(), 1);
    }

    #[test]
    fn test_fixed_serialization_size_ClockControl() {
        assert_eq!(ClockControl::new().fixed_serialization_size(), 10);
    }

    #[test]
    fn test_fixed_serialization_size_Heartbeat() {
        assert_eq!(Heartbeat::new().fixed_serialization_size(), 0);
    }

    #[test]
    fn test_fixed_serialization_size_Announce() {
        assert_eq!(Announce::new().fixed_serialization_size(), 23);
    }

    #[test]
    fn test_fixed_serialization_size_AnnounceService() {
        assert_eq!(AnnounceService::new().fixed_serialization_size(), 1);
    }

    #[test]
    fn test_fixed_serialization_size_RSSI() {
        assert_eq!(RSSI::new().fixed_serialization_size(), 4);
    }

    #[test]
    fn test_fixed_serialization_size_Sms() {
        assert_eq!(Sms::new().fixed_serialization_size(), 2);
    }

    #[test]
    fn test_fixed_serialization_size_SmsTx() {
        assert_eq!(SmsTx::new().fixed_serialization_size(), 6);
    }

    #[test]
    fn test_fixed_serialization_size_SmsRx() {
        assert_eq!(SmsRx::new().fixed_serialization_size(), 0);
    }

    #[test]
    fn test_fixed_serialization_size_SmsState() {
        assert_eq!(SmsState::new().fixed_serialization_size(), 5);
    }

    #[test]
    fn test_fixed_serialization_size_TextMessage() {
        assert_eq!(TextMessage::new().fixed_serialization_size(), 0);
    }

    #[test]
    fn test_fixed_serialization_size_IridiumMsgRx() {
        assert_eq!(IridiumMsgRx::new().fixed_serialization_size(), 24);
    }

    #[test]
    fn test_fixed_serialization_size_IridiumMsgTx() {
        assert_eq!(IridiumMsgTx::new().fixed_serialization_size(), 4);
    }

    #[test]
    fn test_fixed_serialization_size_IridiumTxStatus() {
        assert_eq!(IridiumTxStatus::new().fixed_serialization_size(), 3);
    }

    #[test]
    fn test_fixed_serialization_size_ExtendedRSSI() {
        assert_eq!(ExtendedRSSI::new().fixed_serialization_size(), 5);
    }

    #[test]
    fn test_fixed_serialization_size_LblRange() {
        assert_eq!(LblRange::new().fixed_serialization_size(), 5);
    }

    #[test]
    fn test_fixed_serialization_size_LblBeacon() {
        assert_eq!(LblBeacon::new().fixed_serialization_size(), 23);
    }

    #[test]
    fn test_fixed_serialization_size_EstimatedState() {
        assert_eq!(EstimatedState::new().fixed_serialization_size(), 88);
    }

    #[test]
    fn test_fixed_serialization_size_NavigationUncertainty() {
        assert_eq!(NavigationUncertainty::new().fixed_serialization_size(), 56);
    }

    #[test]
    fn test_fixed_serialization_size_DesiredPath() {
        assert_eq!(DesiredPath::new().fixed_serialization_size(), 56);
    }

    #[test]
    fn test_fixed_serialization_size_PathControlState() {
        assert_eq!(PathControlState::new().fixed_serialization_size(), 81);
    }
}

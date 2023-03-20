#[cfg(test)]
mod tests {
    use crate::AnnounceService::AnnounceService;
    use crate::CacheControl::CacheControl;
    use crate::ClockControl::ClockControl;
    use crate::CpuUsage::CpuUsage;
    use crate::DesiredPath::DesiredPath;
    use crate::EntityActivationState::EntityActivationState;
    use crate::EntityInfo::EntityInfo;
    use crate::EntityList::EntityList;
    use crate::EntityState::EntityState;
    use crate::EstimatedState::EstimatedState;
    use crate::ExtendedRSSI::ExtendedRSSI;
    use crate::Heartbeat::Heartbeat;
    use crate::IridiumMsgRx::IridiumMsgRx;
    use crate::IridiumMsgTx::IridiumMsgTx;
    use crate::IridiumTxStatus::IridiumTxStatus;
    use crate::LblBeacon::LblBeacon;
    use crate::LblRange::LblRange;
    use crate::LogBookControl::LogBookControl;
    use crate::LogBookEntry::LogBookEntry;
    use crate::LoggingControl::LoggingControl;
    use crate::Message::Message;
    use crate::MsgList::MsgList;
    use crate::NavigationUncertainty::NavigationUncertainty;
    use crate::PathControlState::PathControlState;
    use crate::QueryEntityActivationState::QueryEntityActivationState;
    use crate::QueryEntityInfo::QueryEntityInfo;
    use crate::QueryEntityState::QueryEntityState;
    use crate::ReplayControl::ReplayControl;
    use crate::RestartSystem::RestartSystem;
    use crate::SimulatedState::SimulatedState;
    use crate::Sms::Sms;
    use crate::SmsRx::SmsRx;
    use crate::SmsState::SmsState;
    use crate::SmsTx::SmsTx;
    use crate::StorageUsage::StorageUsage;
    use crate::TextMessage::TextMessage;
    use crate::TransportBindings::TransportBindings;
    use crate::VehicleOperationalLimits::VehicleOperationalLimits;
    use crate::RSSI::RSSI;
    use Announce::Announce;

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

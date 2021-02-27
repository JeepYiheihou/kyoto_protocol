use crate::flow::flow_type::{ FlowType, RetFlowType };

pub trait FlowHandler {
    fn handle_flow(&mut self, flow: FlowType) -> crate::Result<RetFlowType>;
}
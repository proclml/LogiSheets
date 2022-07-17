use super::{CalcValue, CalcVertex, Value};
use crate::calc_engine::connector::Connector;

pub fn calc<C>(args: Vec<CalcVertex>, fetcher: &mut C) -> CalcVertex
where
    C: Connector,
{
    todo!()
}

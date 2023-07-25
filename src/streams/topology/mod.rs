use self::{stream::Stream, table::Table, global_table::GlobalTable};

pub mod stream;
pub mod table;
pub mod global_table;

pub struct Topology {

}

impl Topology {
    fn new() -> Self {
        Topology{}
    }

    fn stream(topic: String) -> Stream {
        Stream {  }
    }

    fn table(topic: String) -> Table {
        Table {}
    }

    fn global_table(topic: String) -> GlobalTable {
        GlobalTable {}
    }
}
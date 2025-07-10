#[derive(Clone, Debug)]
struct Table {
    id: u32,
    name: String,
    structure: TableStructure,
    pages: Vec<Page>,
}

#[derive(Clone, Debug)]
struct TableStructure {
    column_names: Vec<String>,
    column_types: Vec<std::any::TypeId>,
}

#[derive(Clone, Debug)]
struct Page {
    rows: Vec<String>,
}

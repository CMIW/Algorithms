pub struct SparseSet<T> {
    sparse: Vec<Option<usize>>, // sparse's content is the index into dense
    dense: Vec<usize>,          // dense's content is the id (for fixup)
    payload: Vec<T>,            // payload's content is whatever we need
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            payload: Vec::new(),
        }
    }

    pub fn insert(&mut self, id: usize, value: T) {
        if id >= self.sparse.len() {
            self.sparse.resize(id + 1, None);
        }
        self.dense.push(id);
        self.payload.push(value);
        self.sparse[id] = Some(self.payload.len() - 1);
    }

    pub fn remove(&mut self, id: usize) -> Option<T> {
        let dense_idx = (*self.sparse.get(id)?)?;
        let last = self.dense.len() - 1;

        self.dense.swap(dense_idx, last);
        self.payload.swap(dense_idx, last);

        let moved_id = self.dense[dense_idx];
        self.sparse[moved_id] = Some(dense_idx);

        self.dense.pop();
        Some(self.payload.pop().unwrap())
    }

    pub fn contains(&self, id: usize) -> bool {
        self.sparse.get(id).copied().flatten().is_some()
    }
}

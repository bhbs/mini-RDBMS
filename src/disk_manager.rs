use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Seek, SeekFrom, Write},
    path::Path,
};

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct PageId(pub u64);

pub const PAGE_SIZE: usize = 4096;

impl DiskManager {
    pub fn new(heap_file: File) -> Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    pub fn open(data_file_path: impl AsRef<Path>) -> Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(data_file_path)?;

        Self::new(heap_file)
    }

    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.0;
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.write_all(data)
    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.0;
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.read_exact(data)
    }
}

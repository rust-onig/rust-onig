use std::mem::transmute;
use std::ops::Index;
use std::iter::Iterator;
use onig_sys;

#[repr(C)]
#[derive(Debug)]
pub struct CaptureTreeNode {
    raw: onig_sys::OnigCaptureTreeNode
}

impl CaptureTreeNode {
    pub fn group(&self) -> usize {
        self.raw.group as usize
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.raw.beg as usize, self.raw.end as usize)
    }

    pub fn len(&self) -> usize {
        self.raw.num_childs as usize
    }

    pub fn childs<'t>(&'t self) -> CaptureTreeNodeIter<'t> {
        CaptureTreeNodeIter { idx: 0, node: self }
    }
}

impl Index<usize> for CaptureTreeNode {
    type Output = CaptureTreeNode;

    fn index(&self, index: usize) -> &CaptureTreeNode {
        if index >= self.len() {
            panic!("capture tree node index overflow")
        }
        unsafe {
            transmute(*self.raw.childs.offset(index as isize))
        }
    }
}

#[derive(Debug)]
pub struct CaptureTreeNodeIter<'t> {
    idx: usize,
    node: &'t CaptureTreeNode
}

impl<'t> Iterator for CaptureTreeNodeIter<'t> {
    type Item = &'t CaptureTreeNode;

    fn next(&mut self) -> Option<&'t CaptureTreeNode> {
        if self.idx < self.node.len() {
            self.idx += 1;
            Some(&self.node[self.idx - 1])
        } else {
            None
        }
    }
}

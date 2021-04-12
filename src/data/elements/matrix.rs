use crate::constant::utility::UUID_PLACEHOLDER;
use crate::data::elements::landing_page::LandingPage;
use crate::data::elements::offer::Offer;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use traversal::{Bft, DftLongestPaths};
use uuid::Uuid;

// pub type WrappedMatrix = Arc<RwLock<Matrix>>;
// pub type WrappedMatrices = Arc<RwLock<Vec<Vec<WrappedMatrix>>>>;

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.value.id.as_ref() == other.value.id.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matrix {
    pub children_groups: Vec<Vec<Self>>,
    pub value: MatrixValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatrixValue {
    pub id: Arc<Uuid>,
    pub parent_matrix: Option<Arc<MatrixValue>>,
    pub group_idx: usize,
    pub item_idx: usize,
    pub depth: usize,
    pub data: MatrixData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatrixData {
    Offer(Offer),
    LandingPage(LandingPage),
    Source,
    Void,
}

impl MatrixValue {
    pub fn child_depth(&self) -> usize {
        self.depth + 1
    }
}

impl Matrix {
    pub fn new_item_idx(&self, from_group_idx: usize) -> Option<usize> {
        if let Some(g) = self.children_groups.get(from_group_idx) {
            Some(g.len())
        } else {
            None
        }
    }

    pub fn new_group_idx(&self) -> usize {
        self.children_groups.len()
    }

    pub fn synchronize_matrix_child_groups(&mut self) -> Result<(), String> {
        if let MatrixData::LandingPage(lp) = self.data() {
            let total_child_groups = self.children_groups.len();
            let num_of_ctas = lp.number_of_calls_to_action;
            if total_child_groups == num_of_ctas as usize {
                return Ok(());
            } else if total_child_groups > num_of_ctas as usize {
                for i in (num_of_ctas as usize..total_child_groups).rev() {
                    self.children_groups.remove(i);
                }
            } else if total_child_groups < num_of_ctas as usize {
                for i in total_child_groups..num_of_ctas as usize {
                    self.children_groups.push(vec![Matrix::void(
                        Some(Arc::new(self.value.clone())),
                        i,
                        0,
                        self.depth() + 1,
                    )])
                }
            }

            if let MatrixData::LandingPage(lp) = self.data() {
                if self.children_groups.len() == lp.number_of_calls_to_action as usize {
                    Ok(())
                } else {
                    Err("Synchronize Failed: g54sdfg".to_string())
                }
            } else {
                Err(String::from("Not a landing page"))
            }
        } else {
            Err(String::from("Not a landing page"))
        }
    }

    pub fn root_synchronize_landing_page_child_groups(&mut self) -> Result<(), String> {
        let mut max_num_ctas = 0usize;
        let mut num_offer_groups = 0usize;

        for item in self.children_groups.get(0).unwrap() {
            if let MatrixData::LandingPage(lp) = item.data() {
                if lp.number_of_calls_to_action as usize > max_num_ctas {
                    max_num_ctas = lp.number_of_calls_to_action as usize;
                }
            }
        }

        num_offer_groups = self.children_groups.len() - 1;
        if max_num_ctas == num_offer_groups {
            return Ok(());
        } else if max_num_ctas > num_offer_groups {
            let difference_to_add = max_num_ctas - num_offer_groups;
            for i in 0..difference_to_add {
                self.children_groups.push(vec![Matrix::void(
                    Some(Arc::new(self.value.clone())),
                    i + num_offer_groups,
                    0,
                    self.depth() + 1,
                )])
            }
        } else if max_num_ctas < num_offer_groups {
            for i in (max_num_ctas..num_offer_groups).rev() {
                self.children_groups.remove(i);
            }
        }

        let mut max_num_ctas = 0usize;
        let mut num_offer_groups = 0usize;

        for item in self.children_groups.get(0).unwrap() {
            if let MatrixData::LandingPage(lp) = item.data() {
                if lp.number_of_calls_to_action as usize > max_num_ctas {
                    max_num_ctas = lp.number_of_calls_to_action as usize;
                }
            }
        }

        num_offer_groups = self.children_groups.len() - 1;

        if num_offer_groups == max_num_ctas {
            Ok(())
        } else {
            Err("Synchronization failed:FV534 ".to_string())
        }
    }

    pub fn search_next_depth<'a, I>(
        i: I,
        target: &Uuid,
        target_depth: usize,
    ) -> Result<&'a mut Matrix, String>
    where
        I: Iterator<Item = &'a mut Matrix>,
    {
        let mut cache = vec![];

        for item in i.map(|s| s).collect::<Vec<_>>() {
            if item.value.depth == target_depth {
                if target == item.value.id.as_ref() {
                    return Ok(item);
                } else {
                    cache.push(item);
                }
            } else {
                cache.push(item);
            }
        }

        let mut iter = vec![];

        for item in cache {
            for group in item.children_groups.iter_mut() {
                for group_item in group {
                    iter.push(group_item)
                }
            }
        }

        if iter.is_empty() {
            let depth_count = target_depth;
            let msg = format!("No more child nodes: {} depth", depth_count);
            return Err(msg);
        }

        Matrix::search_next_depth(iter.into_iter(), target, target_depth + 1)
    }

    pub fn get_mut_depth_target_lock(
        &mut self,
        target: &Uuid,
        depth: usize,
    ) -> Option<&mut Matrix> {
        None
    }

    pub fn depth_target_lock(&self, target: &Uuid, depth: usize) -> Option<MatrixValue> {
        let iter = Bft::new(self, |node| node.children_groups.iter().flatten());
        let mut iter = iter.map(|(depth, node)| (depth, &node.value));

        while let Some((depth_found, item)) = iter.next() {
            if depth_found == depth {
                if item.id.as_ref() == target {
                    return Some(item.clone());
                }
            }
        }
        None
    }

    pub fn target_lock(&self, target: &Uuid) -> Option<MatrixValue> {
        let iter = DftLongestPaths::new(self, |s| s.children_groups.iter().flatten());
        let mut iter = iter.map(|s| s.iter().map(|s| &s.value).collect::<Vec<_>>());

        while let Some(path) = iter.next() {
            for item in path {
                if item.id.as_ref() == target {
                    return Some(item.clone());
                }
            }
        }
        None
    }

    pub fn has_children_in_groups(&self) -> bool {
        let iter = Bft::new(self, |s| self.children_groups.iter().flatten());

        let mut iter = iter.map(|(d, m)| (d, m.value.clone()));

        while let Some((depth, node)) = iter.next() {
            if node.item_idx > 0 {
                return true;
            }
        }
        false
    }

    pub fn max_depth_exceeded(&self) -> bool {
        let iter = DftLongestPaths::new(self, |s| s.children_groups.iter().flatten());
        let mut iter = iter.map(|s| s.iter().map(|s| s.value.clone()).collect::<Vec<_>>());

        for path in iter.next() {
            if path.len() > 9 {
                return true;
            }
        }
        false
    }

    pub fn get_max_depth(&self) -> usize {
        let iter = DftLongestPaths::new(self, |s| s.children_groups.iter().flatten());
        let mut iter = iter.map(|s| s.iter().map(|s| s.value.clone()).collect::<Vec<_>>());

        let mut max = 0usize;
        for path in iter.next() {
            if path.len() > max {
                max = path.len();
            }
        }
        max
    }

    pub fn get_mut_flattened_children(&mut self) -> Vec<&mut Matrix> {
        self.children_groups
            .iter_mut()
            .flatten()
            .map(|s| s)
            .collect::<Vec<_>>()
    }

    pub fn get_flattened_children(&self) -> Vec<&Matrix> {
        self.children_groups
            .iter()
            .flatten()
            .map(|s| s)
            .collect::<Vec<_>>()
    }

    pub fn get_parent_node(&self) -> Option<Arc<MatrixValue>> {
        if let Some(p) = &self.value.parent_matrix {
            Some(arc!(p))
        } else {
            None
        }
    }
    pub fn item_idx(&self) -> usize {
        self.value.item_idx
    }

    pub fn group_idx(&self) -> usize {
        self.value.group_idx
    }

    pub fn parent_id(&self) -> Option<Uuid> {
        if let Some(n) = self.get_parent_node() {
            Some(n.id.as_ref().clone())
        } else {
            None
        }
    }

    pub fn id(&self) -> &Uuid {
        self.value.id.as_ref()
    }

    pub fn data(&self) -> &MatrixData {
        &self.value.data
    }

    pub fn depth(&self) -> usize {
        self.value.depth
    }

    pub fn source() -> Self {
        let id = Arc::new(Uuid::new_v4());

        let mut matrix = Self {
            children_groups: vec![vec![]],
            value: MatrixValue {
                id,
                parent_matrix: None,
                group_idx: 0,
                item_idx: 0,
                depth: 0,
                data: MatrixData::Source,
            },
        };

        let parent = Arc::new(matrix.value.clone());

        let d = matrix.depth() + 1;

        let child = Matrix::void(Some(parent), 0, 0, d);

        matrix.children_groups.get_mut(0).map(|s| s.push(child));
        matrix
    }

    pub fn transform_void(&mut self, new: Transform) {
        match new {
            Transform::Offer(o) => self.value.data = MatrixData::Offer(o),
            Transform::Lander(lp) => {
                let ctas = lp.number_of_calls_to_action;
                self.value.data = MatrixData::LandingPage(lp);
                self.root_synchronize_landing_page_child_groups();
            }
        }
    }

    pub fn void(
        parent_matrix: Option<Arc<MatrixValue>>,
        group_idx: usize,
        item_idx: usize,
        depth: usize,
    ) -> Self {
        let id = Arc::new(Uuid::new_v4());

        Self {
            children_groups: vec![vec![]],
            value: MatrixValue {
                id,
                parent_matrix,
                group_idx,
                item_idx,
                depth,
                data: MatrixData::Void,
            },
        }
    }
}
pub enum Transform {
    Offer(Offer),
    Lander(LandingPage),
}
// impl From<Offer> for Matrix {
//     fn from(o: Offer) -> Self {
//         Self {
//             child_group: vec![],
//             value: Either::Right(o),
//         }
//     }
// }
// impl From<LandingPage> for Matrix {
//     fn from(lp: LandingPage) -> Self {
//         let mut group_list = vec![];
//         for i in 0..lp.number_of_calls_to_action {
//             group_list.push(vec![])
//         }
//
//         Self {
//             child_group: group_list,
//             value: Either::Left(lp),
//         }
//     }
// }

// impl From<LandingPage> for Either<LandingPage, Offer> {
//     fn from(lp: LandingPage) -> Self {
//         Either::Left(lp)
//     }
// }
// impl From<Offer> for Either<LandingPage, Offer> {
//     fn from(off: Offer) -> Self {
//         Either::Right(off)
//     }
// }

// impl Matrix {

//
//     pub fn empty_children(&self) -> Option<Vec<usize>> {
//         if let Either::Left(lp) = &self.value {
//             let mut empty_count = vec![];
//             self.child_group.iter().enumerate().map(|(idx, s)| {
//                 if s.is_empty() {
//                     empty_count.push(idx);
//                 }
//             });
//             if empty_count.is_empty() {
//                 None
//             } else {
//                 Some(empty_count)
//             }
//         } else {
//             None
//         }
//     }
//
//     pub fn add_landing_page(&mut self, group_index: usize, new: LandingPage) {
//         self.child_group
//             .get_mut(group_index)
//             .unwrap()
//             .push(new.into())
//     }
//
//     pub fn add_offer(&mut self, group_index: usize, new: Offer) {
//         self.child_group
//             .get_mut(group_index)
//             .unwrap()
//             .push(new.into())
//     }
//
//     pub fn new(elem: Either<LandingPage, Offer>) -> Self {
//         match elem {
//             Either::Left(lp) => {
//                 let mut child_group = vec![];
//
//                 for i in 0..lp.number_of_calls_to_action {
//                     child_group.push(vec![])
//                 }
//
//                 Self {
//                     child_group,
//                     value: Either::Left(lp),
//                 }
//             }
//
//             Either::Right(off) => Self {
//                 child_group: vec![],
//                 value: Either::Right(off),
//             },
//         }
//     }

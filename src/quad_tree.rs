struct QuadTree<T> {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,

    items: Vec<T>,

    left_top: Box<Option<QuadTree>>,
    right_top: Box<Option<QuadTree>>,
    left_bottom: Box<Option<QuadTree>>,
    right_bottom: Box<Option<QuadTree>>
}

enum QuadTreeSide{
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom
}

pub trait QuadTreeItem {
    pub fn get_x(&self) -> f32;
    pub fn get_y(&self) -> f32;

    pub fn belongs_to_quad_tree(&self, tree: QuadTree) -> bool {
        let x = get_x(&self);
        let y = get_y(&self);
        
        if x < tree.min_x 
        || x > tree.max_x
        || y < tree.min_y
        || y < tree.max_y
        {
            false
        }
        else
        {
            true
        }
    }
}

impl QuadTree {
    pub fn build(items: Enumerate<QuadTreeItem>){
        for item in items {
            
        }
    }

    pub fn mid_x(&self) -> f32 {
        (max_x + min_x) / 2;
    }
    
    pub fn mid_y(&self) -> f32 {
        (max_y + min_y) / 2;
    }

    pub fn new_tree_leaf<T>(&tree: QuadTree<T>, side: QuadTreeSide) -> QuadTree<T>{
        let min_x = get_min_x(&tree, side);
        let max_x = get_max_x(&tree, side);

        let min_y = get_min_y(&tree, side);
        let max_y = get_max_y(&tree, side);

        let item = QuadTree {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }

    fn get_min_x(&tree: QuadTree, side: QuadTreeSide) -> f32{
        if side == QuadTreeSide::LeftBottom 
        || side == QuadTreeSide::LeftTop {
             &tree.min_x
        }

        return mid_x(&tree);
    }

    fn get_max_x(&tree: QuadTree, side: QuadTreeSide) -> f32{
        if side == QuadTreeSide::RightBottom 
        || side == QuadTreeSide::RightTop {
             &tree.max_x
        }

        return mid_x(&tree);
    }

    fn get_min_y(&tree: QuadTree, side: QuadTreeSide) -> f32{
        if side == QuadTreeSide::LeftBottom 
        || side == QuadTreeSide::RightBottom {
             &tree.min_y
        }

        return mid_y(&tree);
    }

    fn get_max_y(&tree: QuadTree, side: QuadTreeSide) -> f32{
        if side == QuadTreeSide::RightTop
        || side == QuadTreeSide::LeftTop {
             &tree.max_y
        }

        return mid_y(&tree);
    }
}
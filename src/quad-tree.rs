struct QuadTree {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,

    left_top: Box<Option<QuadTree>>,
    right_top: Box<Option<QuadTree>>,
    left_bottom: Box<Option<QuadTree>>,
    right_bottom: Box<Option<QuadTree>>
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
}
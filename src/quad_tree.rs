pub mod quad_tree {

    use core::slice::Iter;

    pub struct QuadTree<T: QuadTreeItem> {
        lon_center: f32,
        lat_center: f32,
        lon_quadrant_size: f32,
        lat_quadrant_size:f32,

        lon_min: f32,
        lon_max: f32,
        lat_min: f32,
        lat_max: f32,

        max_depth: i8,

        items: Vec<T>,

        left_top: Option<Box<QuadTree<T>>>,
        right_top: Option<Box<QuadTree<T>>>,
        left_bottom: Option<Box<QuadTree<T>>>,
        right_bottom: Option<Box<QuadTree<T>>>
    }

    enum QuadrantType {
        LeftTop,
        RightTop,
        LeftBottom,
        RightBottom
    }

    impl QuadrantType {
        fn get_quadrant_types() -> Iter<'static, QuadrantType> {
            static QUADRANTTYPES: [QuadrantType; 4] = [QuadrantType::LeftTop , QuadrantType::RightTop, QuadrantType::LeftBottom, QuadrantType::RightBottom];
            QUADRANTTYPES.iter()
        }
    }

    pub trait QuadTreeItem {
        fn get_lat(&self) -> f32;
        fn get_lon(&self) -> f32;

        fn belongs_to_quad_tree<T: QuadTreeItem>(&self, tree: &QuadTree<T>) -> bool {
            self.belongs_to_area(tree.lon_min, tree.lon_max, tree.lat_min, tree.lat_max)
        }

        fn belongs_to_area(&self, min_lon: f32, max_lon: f32, min_lat: f32, max_lat: f32) -> bool {
            let lat = self.get_lat();
            let lon = self.get_lon();
            
            return lat >= min_lat && lat <= max_lat
            && lon >= min_lon && lon <= max_lon;      
        }
    }

    struct BoundingBox {
        lon_min: f32,
        lon_max: f32,
        lat_min: f32,
        lat_max: f32,
    }

    impl BoundingBox {
        pub fn lon_center(&self) -> f32 {
             self.lon_min + self.lon_size() / 2.0
        }

        pub fn lat_center(&self) -> f32 {
             self.lat_min + self.lat_size() / 2.0
        }

        pub fn lon_size(&self) -> f32 {
            self.lon_max - self.lon_min
        }

        pub fn lat_size(&self) -> f32 {
            self.lat_max - self.lat_min
        }
    }

    impl<T:QuadTreeItem> QuadTree<T> {

        pub fn add_item_to_tree(&mut self, item: T) -> bool {
            if !item.belongs_to_quad_tree(self) {
                return false
            }

            if self.max_depth <= 0 {
                self.items.push(item);
                return true
            }
            
            for quadrant_type in QuadrantType::get_quadrant_types() {
                if !self.item_belongs_to_subquadrant(&item, quadrant_type){
                    continue;
                }
                
                let sub_quadrant = self.get_subquadrant(quadrant_type);

                if let None = sub_quadrant {
                    let quadrant_coordinates = self.get_quadrant_bounding_box(quadrant_type);
                    let lon_size = quadrant_coordinates.lon_size() / 2.0;
                    let lat_size = quadrant_coordinates.lat_size() / 2.0;
                    let sub_quadrant = Option::from(Box::new(QuadTree{
                        lon_center: quadrant_coordinates.lon_center(), 
                        lat_center: quadrant_coordinates.lat_center(), 
                        lon_quadrant_size: lon_size, 
                        lat_quadrant_size: lat_size,

                        items: Vec::new(),

                        left_bottom: Option::None,
                        right_bottom: Option:: None,
                        left_top: Option::None,
                        right_top: Option::None,

                        lon_min: quadrant_coordinates.lon_center() - lon_size,
                        lon_max: quadrant_coordinates.lon_center() + lon_size,
                        lat_min: quadrant_coordinates.lat_center() - lat_size,
                        lat_max: quadrant_coordinates.lat_center() + lat_size,

                        max_depth: self.max_depth - 1 }));
                    self.assign_subquadrant(sub_quadrant, quadrant_type);
                };

                if let Some(boxed_subquadrant) = sub_quadrant {
                    let sub_quadrant =  *boxed_subquadrant;
                    return sub_quadrant.add_item_to_tree(item);
                }           
            };        

            // Item doesn't fit into any subquadrant, so we put it into this one
            self.items.push(item);
            return true;
        }    

        fn assign_subquadrant(&mut self, subquadrant: Option<Box<QuadTree<T>>>, quadrant_type: &QuadrantType) {
            match quadrant_type {
                QuadrantType::LeftBottom => self.left_bottom = subquadrant,
                QuadrantType::RightBottom => self.right_bottom = subquadrant,
                QuadrantType::LeftTop => self.left_top = subquadrant,
                QuadrantType::RightTop => self.right_top = subquadrant,
            }
        }

        fn get_subquadrant(&self, quadrant_type: &QuadrantType) -> &Option<Box<QuadTree<T>>> {
            match quadrant_type {
                QuadrantType::LeftBottom => &self.left_bottom,
                QuadrantType::RightBottom => &self.right_bottom,
                QuadrantType::LeftTop => &self.left_top,
                QuadrantType::RightTop => &self.right_top,
            }
        }

        fn item_belongs_to_subquadrant(&self, item: &T, quadrant_type: &QuadrantType) -> bool {
            let b_box = self.get_quadrant_bounding_box(quadrant_type);
            
            item.belongs_to_area(b_box.lon_min, b_box.lon_max, b_box.lat_min, b_box.lat_max)
        }

        fn get_quadrant_bounding_box(&self, quadrant_type: &QuadrantType) -> BoundingBox {
            match quadrant_type {
                QuadrantType::LeftTop => BoundingBox{ lon_min: self.lon_min, lon_max: self.lon_center, lat_min: self.lat_center, lat_max: self.lat_max },
                QuadrantType::RightTop => BoundingBox{ lon_min: self.lon_center, lon_max: self.lon_max, lat_min: self.lat_center, lat_max: self.lat_max },
                QuadrantType::LeftBottom => BoundingBox{ lon_min: self.lon_min, lon_max: self.lon_center, lat_min: self.lat_min, lat_max: self.lat_center },
                QuadrantType::RightBottom => BoundingBox{ lon_min: self.lon_center, lon_max: self.lon_max, lat_min: self.lat_min, lat_max: self.lat_center }
            }
        }
    }
}
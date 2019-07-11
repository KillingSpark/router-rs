extern crate router;
use std::collections::HashMap;

trait Handler {
    type Target;
    fn handle(&mut self,  params: &HashMap<String, String>, msg: u64) -> Self::Target;
}
struct SHandler {}
impl Handler for SHandler {
    type Target = u32;
    fn handle(&mut self, params: &HashMap<String, String>, msg: u64) -> Self::Target{
        println!("{:?}", params);
        10
    }
}

struct ObjKindHandler {}
impl Handler for ObjKindHandler {
    type Target = u32;
    fn handle(&mut self, params: &HashMap<String, String>, msg: u64) -> Self::Target{
        println!("{:?}", params);
        match params.get(&":objectkind".to_owned()).unwrap().as_str() {
            "default" => {20},
            "sessions" => {30},
            _ => 100,
        }
    }
}

struct CollHandler {}
impl Handler for CollHandler {
    type Target = u32;
    fn handle(&mut self, params: &HashMap<String, String>, msg: u64) -> Self::Target{
        println!("{:?}", params);
        match params.get(&":objectkind".to_owned()).unwrap().as_str() {
            "collection" => {
                match params.get(&":objectid".to_owned()).unwrap().as_str(){
                    "default" => {
                        match msg {
                            1 => 10,
                            2 => 20,
                            _ => 300,
                        }
                    }
                    _ => 200,
                }
            },
            _ => 100,
        }
    }
}

struct ItemHandler {}
impl Handler for ItemHandler {
    type Target = u32;
    fn handle(&mut self, params: &HashMap<String, String>, msg: u64) -> Self::Target{
        println!("{:?}", params);
        match params.get(&":objectkind".to_owned()).unwrap().as_str() {
            "collection" => {
                match params.get(&":objectid".to_owned()).unwrap().as_str(){
                    "default" => {
                        match params.get(&":itemid".to_owned()).unwrap().as_str(){
                            "myitemid" => {
                                match msg {
                                    1 => 10,
                                    2 => 20,
                                    _ => 400,
                                }
                            }
                            _ => 300,
                        }
                    }
                    _ => 200,
                }
            },
            _ => 100,
        }
    }
}

fn main() {
    let mut r: router::router::Router<&mut Handler<Target=u32>> = router::router::new_router();
    let mut sh = SHandler{};
    let mut oh = ObjKindHandler{};
    let mut ih = ItemHandler{};
    let mut ch = CollHandler{};

    let route_service = router::route::new_route("/bla/service").unwrap();
    let route_object = router::route::new_route("/blah/service/:objectkind").unwrap();
    let route_item = router::route::new_route("/blah/service/:objectkind/:objectid/:itemid").unwrap();
    let route_coll = router::route::new_route("/blah/service/:objectkind/:objectid").unwrap();
        
    r.add_route(&route_service, &mut sh).unwrap();
    r.add_route(&route_object, &mut oh).unwrap();
    r.add_route(&route_item, &mut ih).unwrap();
    r.add_route(&route_coll, &mut ch).unwrap();

    let route_item_example = router::route::new_route("/blah/service/collection/default/myitemid").unwrap();
    let route_object_example = router::route::new_route("/blah/service/sessions").unwrap();
    let route_coll = router::route::new_route("/blah/service/collection/default").unwrap();

    let (h, ps) = r.route(&route_item_example).unwrap();
    assert_eq!((*h).handle(&ps, 1), 10);
    assert_eq!((*h).handle(&ps, 2), 20);
}
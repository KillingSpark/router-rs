extern crate router;
use std::collections::HashMap;

trait Handler {
    type Target;
    fn handle(
        &mut self,
        r: &mut RouterType,
        params: &HashMap<String, String>,
        msg: u64,
    ) -> Self::Target;
}
struct SHandler {}
impl Handler for SHandler {
    type Target = u32;
    fn handle(
        &mut self,
        _r: &mut RouterType,
        _params: &HashMap<String, String>,
        _msg: u64,
    ) -> Self::Target {
        10
    }
}

struct ObjKindHandler {}
impl Handler for ObjKindHandler {
    type Target = u32;
    fn handle(
        &mut self,
        _r: &mut RouterType,
        params: &HashMap<String, String>,
        _msg: u64,
    ) -> Self::Target {
        match params.get(&":objectkind".to_owned()).unwrap().as_str() {
            "default" => 20,
            "sessions" => 30,
            _ => 100,
        }
    }
}

struct CollHandler {}
impl Handler for CollHandler {
    type Target = u32;
    fn handle(
        &mut self,
        r: &mut RouterType,
        params: &HashMap<String, String>,
        msg: u64,
    ) -> Self::Target {
        let route_string = format!("/blah/service/:objectkind/:objectid/{}", msg);
        let route_item = router::route::new_route(route_string.as_str()).unwrap();
        let ih = std::rc::Rc::new(std::cell::RefCell::new(ItemHandler { id: msg as u32 }));
        r.add_route(&route_item, ih).unwrap();

        match params.get(&":objectkind".to_owned()).unwrap().as_str() {
            "collection" => match params.get(&":objectid".to_owned()).unwrap().as_str() {
                "default" => match msg {
                    1 => 10,
                    2 => 20,
                    _ => 300,
                },
                _ => 200,
            },
            _ => 100,
        }
    }
}

struct ItemHandler {
    id: u32,
}
impl Handler for ItemHandler {
    type Target = u32;
    fn handle(
        &mut self,
        _r: &mut RouterType,
        params: &HashMap<String, String>,
        msg: u64,
    ) -> Self::Target {
        match params.get(&":objectkind".to_owned()).unwrap().as_str() {
            "collection" => match params.get(&":objectid".to_owned()).unwrap().as_str() {
                "default" => match msg {
                    1 => 10 + self.id,
                    2 => 20 + self.id,
                    _ => 300,
                },
                _ => 200,
            },
            _ => 100,
        }
    }
}

type RouterType = router::router::Router<std::rc::Rc<std::cell::RefCell<Handler<Target = u32>>>>;

fn main() {
    let mut r: RouterType = router::router::new_router();
    let sh = std::rc::Rc::new(std::cell::RefCell::new(SHandler {}));
    let oh = std::rc::Rc::new(std::cell::RefCell::new(ObjKindHandler {}));
    let ch = std::rc::Rc::new(std::cell::RefCell::new(CollHandler {}));

    let route_service = router::route::new_route("/bla/service").unwrap();
    let route_object = router::route::new_route("/blah/service/:objectkind").unwrap();
    let route_coll = router::route::new_route("/blah/service/:objectkind/:objectid").unwrap();

    r.add_route(&route_service, sh).unwrap();
    r.add_route(&route_object, oh).unwrap();
    r.add_route(&route_coll, ch).unwrap();

    let route_coll_example = router::route::new_route("/blah/service/collection/default").unwrap();

    let (h, ps) = r.route(&route_coll_example).unwrap();
    let h = h.clone();
    let mut h = h.borrow_mut();

    //this adds a new route to the router
    assert_eq!(h.handle(&mut r, &ps, 1), 10);

    let route_item_example =
        router::route::new_route("/blah/service/collection/default/1").unwrap();
    let (h, ps) = r.route(&route_item_example).unwrap();
    let h = h.clone();
    let mut h = h.borrow_mut();

    //this handler got added in the handle method above
    assert_eq!(h.handle(&mut r, &ps, 2), 21);
}

# Router
This is a generic router for objects in a organized path tree. Think of routers in http-servers that match urls to handlers.

## How does it work
See tests for some more examplesbut the Api is actually this small.

```
struct Beep {
    a: u32,
};

let mut r = router::new_router();

//insert into the router
let mut b1 = Beep { a: 10 };
let route_1 = route::new_route("/beep/1/:param/*").unwrap();
r.add_route(&route_1, &mut b1).unwrap();

let mut b2 = Beep { a: 20 };
let route_2 = route::new_route("/beep/2/:param/*").unwrap();
r.add_route(&route_2, &mut b2).unwrap();

//we can use these routes to get mutable refs to the values in the router
let route_1_request = route::new_route("/beep/1/set_param/this/is/a/longer/path").unwrap();
let route_2_request = route::new_route("/beep/2/set_param/this/is/another/longer/path").unwrap();

let (x1, params1) = r.route(&route_1_request).unwrap();
let (x2, params2) = r.route(&route_2_request).unwrap();

//read out parameters from the route
let param1 = params1.get(&":param".to_owned()).unwrap();
assert_eq!(*param1.as_str(), "set_param");

//do stuff with the references
assert!(x1.a == 10);
x1.a = 100;

assert!(x2.a == 20);
x2.a = 200;

assert!(b2.a == 200);
assert!(b1.a == 100);
```
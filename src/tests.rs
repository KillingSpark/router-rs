use crate::route;
use crate::router;
#[test]
fn test_routing() {
    let mut r: router::Router<u32> = router::new_router();
    let route = route::new_route("/abc/*/:cde").unwrap();
    let route2 = route::new_route("/abcd/*/:cde").unwrap();
    let route_request =
        route::new_route("/abc/this_is_ingnored_by_wildcard/this_is_a_param").unwrap();
    let route2_request =
        route::new_route("/abcd/this_is_ingnored_by_wildcard/this_is_a_param").unwrap();

    let route_not_added_request = route::new_route("/this/does/not/exist").unwrap();
    let route_longer_request = route::new_route("/abcd/wild/param/too/long/path").unwrap();
    let route_shorter_request = route::new_route("/abcd/wild").unwrap();

    let route_with_ending_wildcard = route::new_route("/fgh/:param/*").unwrap();
    let route_with_ending_wildcard_request =
        route::new_route("/fgh/set_param/this/is/aLonger/path").unwrap();

    r.add_route(&route, 20).unwrap();
    r.add_route(&route_with_ending_wildcard, 10).unwrap();

    let (x, params) = r.route(&route_request).unwrap();
    assert!(*x == 20);
    assert_eq!(
        params.get(&":cde".to_owned()),
        Some(&"this_is_a_param".to_owned())
    );

    let (x, params) = r.route(&route_with_ending_wildcard_request).unwrap();
    assert!(*x == 10);
    assert_eq!(
        params.get(&":param".to_owned()),
        Some(&"set_param".to_owned())
    );

    let x = r.route(&route_not_added_request);
    assert_eq!(x, None);
    let x = r.route(&route_longer_request);
    assert_eq!(x, None);
    let x = r.route(&route_shorter_request);
    assert_eq!(x, None);

    let route_longer = route::new_route("/one/:param1/:param2").unwrap();
    let route_longer_req = route::new_route("/one/set1/set2").unwrap();
    let route_shorter = route::new_route("/one/:param1").unwrap();
    let route_shorter_req = route::new_route("/one/set1short").unwrap();

    r.add_route(&route_longer, 123).unwrap();
    r.add_route(&route_shorter, 456).unwrap();

    let (x, p) = r.route(&route_longer_req).unwrap();
    assert!(*x == 123);
    assert_eq!(p.get(&":param1".to_owned()), Some(&"set1".to_owned()));
    assert_eq!(p.get(&":param2".to_owned()), Some(&"set2".to_owned()));
    let (x, p) = r.route(&route_shorter_req).unwrap();
    assert!(*x == 456);
    assert_eq!(p.get(&":param1".to_owned()), Some(&"set1short".to_owned()));

    struct Beep {
        a: u32,
    };

    let mut r = router::new_router();
    let mut b1 = Beep { a: 10 };
    let mut b2 = Beep { a: 20 };

    r.add_route(&route, &mut b1).unwrap();
    let (x1, _) = r.route(&route_request).unwrap();
    assert!(x1.a == 10);
    x1.a = 100;

    r.add_route(&route2, &mut b2).unwrap();
    let (x2, _) = r.route(&route2_request).unwrap();

    assert!(x2.a == 20);
    x2.a = 200;

    assert!(b2.a == 200);
    assert!(b1.a == 100);

    let mut r = router::new_router();
    let route1 = route::new_route("/a/b/c").unwrap();
    let route2 = route::new_route("/a/b/c/d").unwrap();
    assert!(match r.add_route(&route1, 0) {
        Ok(()) => true,
        Err(_) => false,
    });
    assert!(match r.add_route(&route2, 0) {
        Ok(()) => true,
        Err(_) => false,
    });
}

#[test]
fn test_route_collisions() {
    //this checks combinations of collisions that may happen when adding routes.
    //all of these need to error else there is a bug somewhere

    //same path twice
    let mut r = router::new_router();
    let route1 = route::new_route("/a/b/c").unwrap();
    let route2 = route::new_route("/a/b/c").unwrap();
    assert!(match r.add_route(&route1, 0) {
        Ok(()) => true,
        Err(_) => false,
    });
    assert!(match r.add_route(&route2, 0) {
        Ok(()) => false,
        Err(_) => true,
    });

    //same path mixed wildcard and specific
    let mut r = router::new_router();
    let route1 = route::new_route("/a/b/c").unwrap();
    let route2 = route::new_route("/a/*/c").unwrap();
    assert!(match r.add_route(&route1, 0) {
        Ok(()) => true,
        Err(_) => false,
    });
    assert!(match r.add_route(&route2, 0) {
        Ok(()) => false,
        Err(_) => true,
    });

    let mut r = router::new_router();
    let route1 = route::new_route("/a/*/c").unwrap();
    let route2 = route::new_route("/a/b/c").unwrap();
    assert!(match r.add_route(&route1, 0) {
        Ok(()) => true,
        Err(_) => false,
    });
    assert!(match r.add_route(&route2, 0) {
        Ok(()) => false,
        Err(_) => true,
    });

    //same path mixed parameter and specific
    let mut r = router::new_router();
    let route1 = route::new_route("/a/b/c").unwrap();
    let route2 = route::new_route("/a/:param/c").unwrap();
    assert!(match r.add_route(&route1, 0) {
        Ok(()) => true,
        Err(_) => false,
    });
    assert!(match r.add_route(&route2, 0) {
        Ok(()) => false,
        Err(_) => true,
    });

    let mut r = router::new_router();
    let route1 = route::new_route("/a/:param/c").unwrap();
    let route2 = route::new_route("/a/b/c").unwrap();
    assert!(match r.add_route(&route1, 0) {
        Ok(()) => true,
        Err(_) => false,
    });
    assert!(match r.add_route(&route2, 0) {
        Ok(()) => false,
        Err(_) => true,
    });

    //same path mixed wildcard and parameter
    let mut r = router::new_router();
    let route1 = route::new_route("/a/*/c").unwrap();
    let route2 = route::new_route("/a/:param/c").unwrap();
    assert!(match r.add_route(&route1, 0) {
        Ok(()) => true,
        Err(_) => false,
    });
    assert!(match r.add_route(&route2, 0) {
        Ok(()) => false,
        Err(_) => true,
    });

    let mut r = router::new_router();
    let route1 = route::new_route("/a/param/c").unwrap();
    let route2 = route::new_route("/a/*/c").unwrap();
    assert!(match r.add_route(&route1, 0) {
        Ok(()) => true,
        Err(_) => false,
    });
    assert!(match r.add_route(&route2, 0) {
        Ok(()) => false,
        Err(_) => true,
    });
}

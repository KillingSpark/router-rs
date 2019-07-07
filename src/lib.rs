mod router;

#[cfg(test)]
mod tests {
    use crate::router;
    #[test]
    fn test_routing() {
        let mut r: router::Router<u32> = router::new_router();
        let route = router::new_route("/abc/*/:cde").unwrap();
        let route2 = router::new_route("/abcd/*/:cde").unwrap();
        let route_request =
            router::new_route("/abc/this_is_ingnored_by_wildcard/this_is_a_param").unwrap();
        let route2_request =
            router::new_route("/abcd/this_is_ingnored_by_wildcard/this_is_a_param").unwrap();

        let route_not_added_request = router::new_route("/this/does/not/exist").unwrap();
        let route_longer_request = router::new_route("/abcd/wild/param/too/long/path").unwrap();
        let route_shorter_request = router::new_route("/abcd/wild").unwrap();

        r.add_route(&route, 20).unwrap();

        let (x, params) = r.route(&route_request).unwrap();
        assert!(*x == 20);
        assert_eq!(
            params.get(&":cde".to_owned()),
            Some(&"this_is_a_param".to_owned())
        );

        let x = r.route(&route_not_added_request);
        assert_eq!(x, None);
        let x = r.route(&route_longer_request);
        assert_eq!(x, None);
        let x = r.route(&route_shorter_request);
        assert_eq!(x, None);

        struct Beep {
            a: u32,
        };

        let mut r: router::Router<&mut Beep> = router::Router {
            tree: router::Tree::Wildcard(Vec::new()),
        };
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
    }

    #[test]
    fn test_route_collisions() {
        //this checks combinations of collisions that may happen when adding routes.
        //all of these need to error else there is a bug somewhere

        //same path different length
        let mut r = router::new_router();
        let route1 = router::new_route("/a/b/c").unwrap();
        let route2 = router::new_route("/a/b/c/d").unwrap();
        assert!(match r.add_route(&route1, 0) {
            Ok(()) => true,
            Err(_) => false,
        });
        assert!(match r.add_route(&route2, 0) {
            Ok(()) => false,
            Err(_) => true,
        });

        let mut r = router::new_router();
        let route1 = router::new_route("/a/b/c/d").unwrap();
        let route2 = router::new_route("/a/b/c").unwrap();
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
        let route1 = router::new_route("/a/b/c").unwrap();
        let route2 = router::new_route("/a/*/c").unwrap();
        assert!(match r.add_route(&route1, 0) {
            Ok(()) => true,
            Err(_) => false,
        });
        assert!(match r.add_route(&route2, 0) {
            Ok(()) => false,
            Err(_) => true,
        });

        let mut r = router::new_router();
        let route1 = router::new_route("/a/*/c").unwrap();
        let route2 = router::new_route("/a/b/c").unwrap();
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
        let route1 = router::new_route("/a/b/c").unwrap();
        let route2 = router::new_route("/a/:param/c").unwrap();
        assert!(match r.add_route(&route1, 0) {
            Ok(()) => true,
            Err(_) => false,
        });
        assert!(match r.add_route(&route2, 0) {
            Ok(()) => false,
            Err(_) => true,
        });

        let mut r = router::new_router();
        let route1 = router::new_route("/a/:param/c").unwrap();
        let route2 = router::new_route("/a/b/c").unwrap();
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
        let route1 = router::new_route("/a/*/c").unwrap();
        let route2 = router::new_route("/a/:param/c").unwrap();
        assert!(match r.add_route(&route1, 0) {
            Ok(()) => true,
            Err(_) => false,
        });
        assert!(match r.add_route(&route2, 0) {
            Ok(()) => false,
            Err(_) => true,
        });

        let mut r = router::new_router();
        let route1 = router::new_route("/a/param/c").unwrap();
        let route2 = router::new_route("/a/*/c").unwrap();
        assert!(match r.add_route(&route1, 0) {
            Ok(()) => true,
            Err(_) => false,
        });
        assert!(match r.add_route(&route2, 0) {
            Ok(()) => false,
            Err(_) => true,
        });
    }
}

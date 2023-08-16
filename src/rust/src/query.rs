use extendr_api::prelude::*;
use sfconversions::IntoGeom;
use sfconversions::vctrs::as_rsgeo_vctr;
use sfconversions::{vctrs::geom_class, Geom};

use geo::{
    Closest, ClosestPoint, GeodesicBearing, HaversineBearing, HaversineClosestPoint, IsConvex,
    LineInterpolatePoint, LineLocatePoint,
};
use geo_types::{LineString, Point};

// /// Calculate Bearing
// ///
// /// @param x an object of class `point`
// /// @param y for `bearing()` an object of class `point`. For `bearings()` an object of class `rs_POINT`
// ///
// /// @returns
// /// A vector of doubles of the calculated bearing for between x and y
// ///
// /// @export

#[extendr]
fn bearing_haversine(x: List, y: List) -> Doubles {
    let x_cls = x.class().unwrap().next().unwrap();
    let y_cls = y.class().unwrap().next().unwrap();

    if (x_cls != "rs_POINT") || (y_cls != "rs_POINT") {
        panic!("`x` and `y` must be point geometries of class `rs_POINT`");
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let p1: Point = Geom::try_from(xi).unwrap().geom.try_into().unwrap();

                let p2: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();

                p1.haversine_bearing(p2).into()
            }
        })
        .collect::<Doubles>()
}

#[extendr]
fn bearing_geodesic(x: List, y: List) -> Doubles {
    let x_cls = x.class().unwrap().next().unwrap();
    let y_cls = y.class().unwrap().next().unwrap();

    if (x_cls != "rs_POINT") || (y_cls != "rs_POINT") {
        panic!("`x` and `y` must be point geometries of class `rs_POINT`");
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let p1: Point = Geom::try_from(xi).unwrap().geom.try_into().unwrap();

                let p2: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();

                p1.geodesic_bearing(p2).into()
            }
        })
        .collect::<Doubles>()
}

// #[extendr]
// /// Find the closest point
// ///
// /// @param x a single geometry
// /// @param y a `point`
// ///
// /// @export
// fn closest_point(x: Robj, y: Robj) -> Robj {
//     let res = Geom::from(x)
//         .geom
//         .closest_point(&Geom::from(y).geom.try_into().unwrap());

//     match res {
//         Closest::SinglePoint(res) => to_pntr(Geom::from(res)),
//         Closest::Intersection(res) => to_pntr(Geom::from(res)),
//         // id like a better approach here
//         Closest::Indeterminate => Robj::from(extendr_api::NA_LOGICAL),
//     }
// }

#[extendr]
fn closest_point(x: List, y: List) -> Robj {
    // check that y is a point
    let y_cls = y.class().unwrap().next().unwrap();
    if y_cls != "rs_POINT" {
        panic!("`y` must be point geometries of class `rs_POINT`");
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                NULL.into_robj()
            } else {
                let p: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();

                let closest = Geom::try_from(xi).unwrap().geom.closest_point(&p);

                match closest {
                    Closest::SinglePoint(pnt) => Geom::from(pnt).into(),
                    Closest::Intersection(pnt) => Geom::from(pnt).into(),
                    Closest::Indeterminate => NULL.into_robj(),
                }
            }
        })
        .collect::<List>()
        .set_class(sfconversions::vctrs::geom_class("point"))
        .unwrap()
}

#[extendr]
fn closest_point_haversine(x: List, y: List) -> Robj {
    // check that y is a point
    let y_cls = y.class().unwrap().next().unwrap();
    if y_cls != "rs_POINT" {
        panic!("`y` must be point geometries of class `rs_POINT`");
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                NULL.into_robj()
            } else {
                let p: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();

                let closest = Geom::try_from(xi).unwrap().geom.haversine_closest_point(&p);

                match closest {
                    Closest::SinglePoint(pnt) => Geom::from(pnt).into(),
                    Closest::Intersection(pnt) => Geom::from(pnt).into(),
                    Closest::Indeterminate => NULL.into_robj(),
                }
            }
        })
        .collect::<List>()
        .set_class(sfconversions::vctrs::geom_class("point"))
        .unwrap()
}

#[extendr]
fn is_convex(x: List) -> Logicals {
    // check that y is a point
    let x_cls = x.class().unwrap().next().unwrap();
    if x_cls != "rs_LINESTRING" {
        panic!("`y` must be LineString geometries of class `rs_LINESTRING`");
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rbool::na()
            } else {
                LineString::try_from(Geom::try_from(xi).unwrap())
                    .unwrap()
                    .is_convex()
                    .into()
            }
        })
        .collect::<Logicals>()
}

#[extendr]
fn line_interpolate_point(x: List, fraction: Doubles) -> Robj {
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be a `rs_LINESTRING`")
    }

    x.iter()
        .zip(fraction.into_iter())
        .map(|((_, xi), fi)| {
            if xi.is_null() || fi.is_na() || fi.is_infinite() || fi.is_nan() {
                NULL.into_robj()
            } else {
                let l: LineString = Geom::try_from(xi).unwrap().try_into().unwrap();

                let res = l.line_interpolate_point(fi.inner());

                match res {
                    Some(res) => Geom::from(res).into(),
                    None => NULL.into_robj(),
                }
            }
        })
        .collect::<List>()
        .set_class(geom_class("point"))
        .unwrap()
}

#[extendr]
fn locate_point_on_line(x: List, y: List) -> Doubles {
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be an `rs_LINESTRING`")
    } else if !y.inherits("rs_POINT") {
        panic!("`y` must be an `rs_POINT")
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let l: LineString = Geom::try_from(xi).unwrap().geom.try_into().unwrap();

                let p: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();

                l.line_locate_point(&p).into()
            }
        })
        .collect::<Doubles>()
}



use geo::algorithm::LinesIter;
use geo::algorithm::EuclideanLength;
use geo_types::Coord;

#[extendr]
fn split_line(x: Robj, fraction: f64) -> Robj {

    let x = LineString::from(Geom::from(x));
    
    let mut lns = x.lines_iter();
    let mut ln_vec: Vec<Coord> = Vec::new();

    // push the first coord in
    // each subsequent coord will be the end point
    ln_vec.push(lns.nth(0).unwrap().start);


    let total_length = x.euclidean_length();
    let fractional_length = total_length * fraction;
    let mut cum_length = 0_f64;
    
    // Pre-allocate LineString vector with `n` elements
    // iterate up to the first fractional amount collecting 
    // Coords along the way. Once that fractional piece is 
    // identified, create the linestring from Vec<Coord> and push.
    // Reinstantiate a new Vec<Coord> where the first Coord is the 
    // previous endpoint. Then change the target fractional length
    // to the next calculated one. Repeat until the last fractional element 
    // is fetched (which should be the last line segment in the for loop).
    for segment in lns {
        let length = segment.euclidean_length();
        if cum_length + length >= fractional_length {
            let segment_fraction = (fractional_length - cum_length) / length;
            let endpoint = segment.line_interpolate_point(segment_fraction).unwrap().0;
            ln_vec.push(endpoint);
            break;
        }

        cum_length += length;
        ln_vec.push(segment.start);
    }

    let res = LineString::new(ln_vec);
    as_rsgeo_vctr(list!(res.into_geom()), "linestring")

}


#[extendr]
fn segmentize(x: Robj, n: i32) -> Robj {

    let x = LineString::from(Geom::from(x));

    // Convert X into an iterator of `Lines` 
    let mut lns = x.lines_iter();
    
    // Vec to allocate the  new LineString segments Coord Vec
    // will be iterated over at end to create new vecs
    let mut res_coords: Vec<Vec<Coord>> = Vec::with_capacity(3);

    // calculate total length to track cumulative against
    let total_length = x.euclidean_length();

    // tracks total length
    let mut cum_length = 0_f64;

    // calculate the target fraction for the first iteration
    // fraction will change based on each iteration
    // let mut fraction = (1_f64 / (n as f64)) * (idx as f64);
    let segment_prop = 1_f64 / (n as f64);
    let mut fraction = segment_prop;

    // fractional length will change dependent upon which `n` we're on.
    let mut fractional_length = total_length * fraction;    

    // instantiate the first Vec<Coord>
    let mut ln_vec: Vec<Coord> = Vec::new();

    // push the first coord in
    // each subsequent coord will be the end point
    ln_vec.push(lns.nth(0).unwrap().start);

    // iterate through each line segment in the LineString
    for segment in lns {

        let length = segment.euclidean_length();

        // update cumulative length
        cum_length += length;

        if cum_length >= fractional_length {

            let segment_fraction = (fractional_length - cum_length) / length;
            
            // what do we do if the point cannot be unwrapped here? Return None? 
            // Should this trait return Option<MultiLineString>?
            let endpoint = segment.line_interpolate_point(segment_fraction).unwrap().0;

            // add final coord to ln_vec
            ln_vec.push(endpoint);

            // now we drain all elements from the vector into an iterator
            // this will be collected into a vector to be pushed into the 
            // results coord vec of vec
            let to_push = ln_vec.drain(..);

            // now collect & push this vector into the results vector
            res_coords.push(to_push.collect::<Vec<Coord>>());

            // now add the last endpoint as the first coord
            ln_vec.push(endpoint);

            // we need to adjust our fraction and fractional length
            fraction += segment_prop;
            fractional_length = total_length * fraction;

        }

        // push the end coordinate into the Vec<Coord> to continue
        // building the linestring
        ln_vec.push(segment.end);
    }

    // push the last linestring vector which isn't done by the for loop
    res_coords.push(ln_vec);

    let res = res_coords
        .into_iter()
        .map(|xi| Geom::from(LineString::new(xi)))
        .collect::<Vec<Geom>>();

    as_rsgeo_vctr(List::from_values(res), "linestring")

}

// let total_length = self.euclidean_length();
//     let fractional_length = total_length * fraction;
//     let mut cum_length = T::zero();
//     for segment in self.lines() {
//         let length = segment.euclidean_length();
//         if cum_length + length >= fractional_length {
//             let segment_fraction = (fractional_length - cum_length) / length;
//             return segment.line_interpolate_point(segment_fraction);
//         }
//         cum_length += length;
//     }

extendr_module! {
    mod query;
    fn bearing_geodesic;
    fn bearing_haversine;
    fn closest_point;
    fn closest_point_haversine;
    fn is_convex;
    fn line_interpolate_point;
    fn locate_point_on_line;
    fn split_line;
    fn segmentize;
}

pub enum RectIntersection {
    Equal,
    Collision,
    Outside,
    AContainsB,
    BContainsA,
}

pub fn intersection(rect_a: Rect, rect_b: Rect) -> RectIntersection {
    let aminx = rect_a.x;
    let aminy = rect_a.y;
    let amaxx = rect_a.x + rect_a.width;
    let amaxy = rect_a.y + rect_a.height;

    let bminx = rect_b.x;
    let bminy = rect_b.y;
    let bmaxx = rect_b.x + rect_b.width;
    let bmaxy = rect_b.y + rect_b.height;

    if rect_a == rect_b {
        RectIntersection::Equal
    } else if (aminx >= bminx && aminy >= bminy) && (amaxx <= bmaxx && amaxy <= bmaxy) {
        RectIntersection::BContainsA
    } else if (aminx <= bminx && aminy <= bminy) && (amaxx >= bmaxx && amaxy >= bmaxy) {
        RectIntersection::AContainsB
    } else if (aminx < bmaxx && amaxx > bminx) && (aminy < bmaxy && amaxy > bminy) {
        RectIntersection::Collision
    } else {
        RectIntersection::Outside
    }
}

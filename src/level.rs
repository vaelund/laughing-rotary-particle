use std::collections::{HashSet, VecDeque};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use image::{DynamicImage, GenericImageView, Rgba};

#[derive(Component)]
struct Terrain;

pub fn load_level_geo_new(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut level = image::open("assets/testworld.png").unwrap();
    level = level.flipv();
}

/// Walk the perimeter of a pixel group, returning a list of points forming a
/// complete outline of the pixels. This outline composes the coordinates of
/// the "lines" between the pixels. The upper left corner of a pixel has the
/// coordinate offset (0.0, 0.0) and the lower right corner has the coordinate
/// offset (1.0, 1.0).
///
///
/// Example:
/// x inside pixel(of the same color as the start)
/// o outside pixel
/// | perimeter
/// coordinates start at top left
///
/// x | x | o
/// --+---+--
/// x | x | o
/// --+---+--
/// o | o | o
///
/// returns [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (0.0, 0.0)]
///
/// o | o | o
/// --+---+--
/// o | x | x
/// --+---+--
/// o | x | x
///
/// returns [(1.0, 1.0), (3.0, 1.0), (3.0, 3.0), (1.0, 3.0), (1.0, 1.0)]
///
/// o | o | o
/// --+---+--
/// o | x | o
/// --+---+--
/// o | o | o
///     
/// returns [(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0), (1.0, 1.0)]
///
fn walk_pixel_group_perimeter(
    image: &DynamicImage,
    start_pixel_x: u32,
    start_pixel_y: u32,
) -> Vec<Vec2> {
    // we start at the left upper corner of the start_pixel, which by definition is inside the pixel group
    let mut current_pixel_x = start_pixel_x as i32;
    let mut current_pixel_y = start_pixel_y as i32;

    // try to find the first pixel corner that is not borderered by a pixel of the same color
    // from that corner we start our walk
    // if the pixel left of the current pixel is of the same color, we can not walk from the top left to the bottom left (downwards)
    // if the pixel above the current pixel is of the same color, we can not walk from the top left to the top right (rightwards)
    // if the pixel right of the current pixel is of the same color, we can not walk from the top right to the bottom right (downwards)
    // if the pixel below the current pixel is of the same color, we can not walk from the bottom left to the bottom right (rightwards)
    // one of these four cases must be false otherwise our start is inside a group of pixels of the same color and therefore not usable as a start pixel

    // check if the pixel left of the current pixel is of the same color
    let mut left_pixel_x = current_pixel_x - 1;
    let mut left_pixel_y = current_pixel_y;
    let mut left_pixel_is_inner_pixel = is_inner_pixel(
        left_pixel_x as u32,
        left_pixel_y as u32,
        image,
        image.get_pixel(start_pixel_x, start_pixel_y),
    );
    // check if the pixel above the current pixel is of the same color
    let mut top_pixel_x = current_pixel_x;
    let mut top_pixel_y = current_pixel_y + 1;
    let mut top_pixel_is_inner_pixel = is_inner_pixel(
        top_pixel_x as u32,
        top_pixel_y as u32,
        image,
        image.get_pixel(start_pixel_x, start_pixel_y),
    );
    // check if the pixel right of the current pixel is of the same color
    let mut right_pixel_x = current_pixel_x + 1;
    let mut right_pixel_y = current_pixel_y;
    let mut right_pixel_is_inner_pixel = is_inner_pixel(
        right_pixel_x as u32,
        right_pixel_y as u32,
        image,
        image.get_pixel(start_pixel_x, start_pixel_y),
    );
    // check if the pixel below the current pixel is of the same color
    let mut bottom_pixel_x = current_pixel_x;
    let mut bottom_pixel_y = current_pixel_y - 1;
    let mut bottom_pixel_is_inner_pixel = is_inner_pixel(
        bottom_pixel_x as u32,
        bottom_pixel_y as u32,
        image,
        image.get_pixel(start_pixel_x, start_pixel_y),
    );

    return Vec::new();
}

/// Check if a pixel is an inner pixel of a pixel group.
/// A pixel is an inner pixel if it has the same color as the pixel group and
/// is inside the boundaries of the image.
fn is_inner_pixel(x: u32, y: u32, image: &DynamicImage, inner_color: Rgba<u8>) -> bool {
    if x < 0 || y < 0 || x > image.width() || y > image.height() {
        return false;
    }

    let pixel = image.get_pixel(x, y);
    pixel == inner_color
}

pub fn load_level_geo(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut level = image::open("assets/testworld.png").unwrap();
    level = level.flipv();

    let (level_width, level_height) = level.dimensions();

    // Make black pixels transparent and create a collider for each line
    for y in 0..level_height {
        let mut current_color = Rgba([0, 0, 0, 255]);
        let mut line_start = Vec2::new(0.0, y as f32);

        for x in 0..level_width {
            let pixel = level.get_pixel(x, y);

            // Check if the pixel is not black
            if pixel != current_color {
                if current_color != Rgba([0, 0, 0, 255]) {
                    // Create a sprite with collider for the last color
                    let line_width = x as f32 - line_start.x;
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgb(
                                    current_color.0[0] as f32 / 255.0,
                                    current_color.0[1] as f32 / 255.0,
                                    current_color.0[2] as f32 / 255.0,
                                ),
                                custom_size: Some(Vec2::new(line_width, 1.0)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                line_start.x + line_width / 2.0,
                                line_start.y + 0.5, // Adjust as needed
                                0.0,
                            )),
                            ..default()
                        },
                        Terrain,
                        Collider::cuboid(line_width / 2.0, 0.5),
                    ));
                }

                // Start a new line
                line_start = Vec2::new(x as f32, y as f32);
                current_color = pixel;
            }
        }

        // Create a collider for the last line in the row
        if current_color != Rgba([0, 0, 0, 255]) {
            let line_width = (level_width - 1) as f32 - line_start.x;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(
                            current_color.0[0] as f32 / 255.0,
                            current_color.0[1] as f32 / 255.0,
                            current_color.0[2] as f32 / 255.0,
                        ),
                        custom_size: Some(Vec2::new(line_width, 1.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        line_start.x + line_width / 2.0,
                        line_start.y + 0.5, // Adjust as needed
                        0.0,
                    )),
                    ..default()
                },
                Terrain,
                Collider::cuboid(line_width / 2.0, 0.5),
            ));
        }
    }

    // add the bitmap as a large background sprite
    // offset the sprite by half the size of the image to center it
    // flip the image vertically to match the coordinate system
    // let offset = Vec3::new(level.width() as f32, level.height() as f32, 0.0) / 2.0;
    // commands.spawn(SpriteBundle {
    //     texture: images.add(bevy::render::texture::Image::from_dynamic(
    //         level.flipv(),
    //         true,
    //     )),
    //     transform: Transform {
    //         translation: offset,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
}

pub fn load_level_geo_old(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let level = image::open("assets/testworld.png").unwrap();

    let mut visited = HashSet::new();
    let mut groups: Vec<Vec<Vec2>> = Vec::new();

    // Iterate over non-black pixels and create entities with colliders
    for (x, y, pixel) in level.pixels() {
        if (pixel.0[0] > 0 || pixel.0[1] > 0 || pixel.0[2] > 0) && !visited.contains(&(x, y)) {
            let group = find_group(&level, x, y, &mut visited);
            groups.push(group);
        }
    }

    // Create a convex hull for each group
    for group in groups {
        let mut outer_pixels = find_outer_pixels(&level, &group);
        // simplyfiy the polygon by removing points that are too close to each other
        outer_pixels = simplify_polygon(&outer_pixels, 2.0);
        // add the first coordinate again as the last to close the polygon
        outer_pixels.push(outer_pixels[0]);
        info!("Outer pixels: {:?}", outer_pixels.len());
        spawn_collider_from_vertices_polyline(&mut commands, &outer_pixels)
    }

    // add the bitmap as a large background sprite
    // offset the sprite by half the size of the image to center it
    let offset = Vec3::new(level.width() as f32, level.height() as f32, 0.0) / 2.0;
    commands.spawn(SpriteBundle {
        texture: images.add(bevy::render::texture::Image::from_dynamic(level, true)),
        transform: Transform {
            translation: offset,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_collider_from_vertices_polyline(commands: &mut Commands, vertices: &[Vec2]) {
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        Collider::polyline(vertices.into(), None),
    ));
}

fn spawn_collider_from_vertices_convex_decompose(commands: &mut Commands, vertices: &[Vec2]) {
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        Collider::convex_decomposition_with_params(vertices, &[], &VHACDParameters::default()),
    ));
}

// Function to simplify a polygon by removing points that are too close to each other
fn simplify_polygon(polygon: &[Vec2], threshold: f32) -> Vec<Vec2> {
    let mut simplified_polygon = Vec::new();
    let mut last_point = polygon[0];
    simplified_polygon.push(last_point);

    for &point in polygon {
        let distance = (point - last_point).length();
        if distance > threshold {
            simplified_polygon.push(point);
            last_point = point;
        }
    }

    simplified_polygon
}

// Modified flood-fill algorithm to find non-black pixels and group them
fn find_group(
    image: &DynamicImage,
    start_x: u32,
    start_y: u32,
    visited: &mut HashSet<(u32, u32)>,
) -> Vec<Vec2> {
    let mut group = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_x as i32, start_y as i32));

    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x as u32, y as u32)) {
            continue;
        }

        visited.insert((x as u32, y as u32));

        let is_non_black_pixel = is_non_black_pixel(image, x, y);
        if is_non_black_pixel {
            group.push(Vec2::new(x as f32, y as f32));
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && ny >= 0 && nx < image.width() as i32 && ny < image.height() as i32 {
                    let pixel = image.get_pixel(nx as u32, ny as u32);
                    if pixel.0[0] > 0 || pixel.0[1] > 0 || pixel.0[2] > 0 {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    group
}

// Function to determine if a pixel is a non-black pixel
fn is_non_black_pixel(image: &DynamicImage, x: i32, y: i32) -> bool {
    let pixel = image.get_pixel(x as u32, y as u32);
    pixel.0[0] > 0 || pixel.0[1] > 0 || pixel.0[2] > 0
}

// Function to find the outer pixels of a group and order them in counter-clockwise direction
fn find_outer_pixels(image: &DynamicImage, group: &[Vec2]) -> Vec<Vec2> {
    let centroid = calculate_centroid(group);

    // Filter and sort points by angle in counter-clockwise order
    let mut points_with_angles: Vec<(f32, Vec2)> = group
        .iter()
        .filter(|&&point| is_outer_pixel(image, point.x as i32, point.y as i32))
        .map(|&point| {
            let angle = calculate_angle(&centroid, &point);
            (angle, point)
        })
        .collect();

    points_with_angles.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut outer_pixels = Vec::new();
    outer_pixels.extend(points_with_angles.into_iter().map(|(_, point)| point));
    outer_pixels
}

// Function to determine if a pixel is an outer boundary pixel
fn is_outer_pixel(image: &DynamicImage, x: i32, y: i32) -> bool {
    // Check if the pixel is at the image boundary or if it's a black pixel
    if x == 0 || y == 0 || x == (image.width() as i32 - 1) || y == (image.height() as i32 - 1) {
        return true;
    }

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0 || nx >= image.width() as i32 || ny >= image.height() as i32 {
                return true; // Pixel is at the image boundary
            }

            let pixel = image.get_pixel(nx as u32, ny as u32);
            if pixel.0[0] == 0 && pixel.0[1] == 0 && pixel.0[2] == 0 {
                return true; // Pixel is a black pixel
            }
        }
    }

    false
}

// Function to calculate the centroid of a group of points
fn calculate_centroid(group: &[Vec2]) -> Vec2 {
    let mut centroid = Vec2::ZERO;
    for &point in group {
        centroid += point;
    }
    centroid / group.len() as f32
}

// Function to calculate the angle between two points in counter-clockwise direction
fn calculate_angle(origin: &Vec2, point: &Vec2) -> f32 {
    let dx = point.x - origin.x;
    let dy = point.y - origin.y;
    dy.atan2(dx)
}

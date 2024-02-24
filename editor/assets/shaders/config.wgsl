const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(0.0, 0.2, .0, 1.0);

struct FernResult {
    pos: vec3<f32>,
    normal: vec3<f32>,
    ao: f32,
    uv: vec2<f32>,
    tangent: vec4<f32>
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
};


fn fern_vertices(t: f32, vertex: Vertex) -> FernResult {
    let vertices_per_leaf: u32 = u32(12);

    var fi = f32(vertex.vertex_index % vertices_per_leaf) - 1.0;
    let vpl3 = f32(vertices_per_leaf - 3u);
    if fi <= 0.0 {
        fi = 0.0;
    } else if fi >= vpl3 {
        fi = vpl3;
    }
    var raw_leaf = floor(f32(vertex.vertex_index) / f32(vertices_per_leaf));
#ifdef RENDER_BACKFACE
    let leaf = floor(raw_leaf / 2.0);
#else 
    let leaf = raw_leaf;
#endif

    var pos = vertex.position;
    let dist = floor(fi / 2.0);
    let rfi = dist / f32(vertices_per_leaf - 4u) * 2.0;

    // width of the leaf
    // let w = 4.0 / f32(vertices_per_leaf);
    // let shape = 1.0 - rfi;
    let w = 10.0 / f32(vertices_per_leaf);
    let shape = 1.0;
    // length of the leaf; varies slightly per leaf
    let l = 12.0 / f32(vertices_per_leaf) + sin(leaf + 100.0) * 0.1;
    // first leafs are bent more
    let bendStrength = -2.0 / f32(vertices_per_leaf);

    //let tooth = (f32(vertex.vertex_index) / 2.0) % 2.0;
    //if tooth <= 0.1 || tooth >= 1.4 {
        //w = w * 0.1;
    //}

    let golden_angle = 2.39996322972865332;
    var yaw = golden_angle * leaf;

    let time = t - (yaw % radians(360.0)) - dist * 0.3;
    let wind = sin(time) - sin(time / 2.0) + sin(time / 4.0) - sin(time / 8.0);

    let pitch = -0.9 + leaf * 0.02; // + wind * 0.08;
    yaw += wind * 0.03;

    let lr = fi % 2.0 - 0.5;
    pos.x = -lr * shape * w;
    let bentPitch = pitch + dist * (bendStrength + wind * 0.01);
    pos.y += cos(bentPitch) * dist * l;
    pos.z += sin(bentPitch) * dist * l;

    // rotate around the y axis
    let yaw_rotation = mat2x2<f32>(cos(yaw), sin(yaw), -sin(yaw), cos(yaw));
    let r = pos.xz * yaw_rotation;
    pos.x = r.x;
    pos.z = r.y;

    var normal_rot = radians(90.0);
#ifdef RENDER_BACKFACE
    if (raw_leaf % 2.0) <= 0.5 {
        normal_rot = radians(-90.0);
        pos.y -= 0.001;
    }
#endif

    var normal = vec3<f32>(0.0, cos(bentPitch + normal_rot), sin(bentPitch + normal_rot));
    let rr = normal.xz * yaw_rotation;
    normal.x = rr.x;
    normal.z = rr.y;

    var tangent = vec4<f32>(0.0, cos(bentPitch), sin(bentPitch), 1.0);
    let rrr = tangent.xz * yaw_rotation;
    tangent.x = rrr.x;
    tangent.z = rrr.y;

    pos *= 0.2;
    pos.y -= 0.5;

    let ao = clamp(pow(rfi, 3.0) * 3.0 - 0.15, 0.04, 1.0);
    let uv = vec2<f32>(1.0 - rfi, lr + 0.5);

    return FernResult(pos, normal, ao, uv, tangent);
}
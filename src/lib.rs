use cimvr_common::{
    render::{Mesh, Primitive, Render, UploadMesh, MeshHandle, ShaderSource, ShaderHandle, DEFAULT_VERTEX_SHADER},
    Transform,
};
use cimvr_engine_interface::{make_app_state, pkg_namespace, prelude::*, println};

use crate::obj::obj_lines_to_mesh;

mod obj;

// All state associated with client-side behaviour
struct ClientState;

pub const DODECA_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Dodeca"));
pub const AVATAR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Avatar"));
pub const AVATAR_SHADER: ShaderHandle = ShaderHandle::new(pkg_namespace!("Avatar"));


fn dodo() -> Mesh {
    let dodecahedron = obj_lines_to_mesh(include_str!("assets/dodecahedron.obj"));
    
    dodecahedron
}

fn avatar() -> Mesh {
    let avatar = obj_lines_to_mesh(include_str!("assets/avatar.obj"));

    avatar
}

fn avatar_shader() -> ShaderSource {
    let fragment_src = "
    #version 330
    
    in vec4 color;
    out vec4 out_color;
    
    void main() {
        out_color = vec4(1., 1., 0., 1.);
    }"
        .into();
    ShaderSource {
        vertex_src: DEFAULT_VERTEX_SHADER.to_string(),
        fragment_src,
        id: AVATAR_SHADER,
    }
}

impl UserState for ClientState {
    // Implement a constructor
    fn new(io: &mut EngineIo, _sched: &mut EngineSchedule<Self>) -> Self {
        
        io.send(&UploadMesh { 
            mesh: avatar(),
             id: AVATAR_RDR });
        
        io.send(&avatar_shader());

        // NOTE: We are using the println defined by cimvr_engine_interface here, NOT the standard library!
        cimvr_engine_interface::println!("This prints");
        std::println!("But this doesn't");

        Self
    }
}

// All state associated with server-side behaviour
struct ServerState;

impl UserState for ServerState {
    // Implement a constructor
    fn new(io: &mut EngineIo, _sched: &mut EngineSchedule<Self>) -> Self {

        let avatar_render = Render {
            id: AVATAR_RDR,
            primitive: Primitive::Triangles,
            limit: None,
            shader: None,
        };

        let ent = io.create_entity();
        io.add_component(ent, &Transform::identity());
        io.add_component(ent, &avatar_render);
        io.add_component(ent, &Synchronized);

        println!("Hello, server!");
        Self
    }
}

// Defines entry points for the engine to hook into.
// Calls new() for the appropriate state.
make_app_state!(ClientState, ServerState);

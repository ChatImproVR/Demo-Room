use cimvr_common::{
    render::{
        Mesh, MeshHandle, Primitive, Render, ShaderHandle, ShaderSource, UploadMesh,
        DEFAULT_VERTEX_SHADER,
    },
    Transform,
};
use cimvr_engine_interface::{make_app_state, pkg_namespace, prelude::*, println};
use obj_reader::obj::obj_lines_to_mesh;
// All state associated with client-side behaviour
struct ClientState;

// Galaga Room
pub const AVATAR_SHADER: ShaderHandle = ShaderHandle::new(pkg_namespace!("Avatar"));
pub const AVATAR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Avatar"));

pub const COUCH_GR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Couch_gr"));
pub const TABLE_GR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Table_gr"));
pub const MUG_GR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Mug_gr"));
pub const TV_GR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("TV_gr"));

fn avatar() -> Mesh {
    let avatar = obj_lines_to_mesh(include_str!("assets/avatar.obj"));
    avatar
}

fn couch_gr() -> Mesh {
    let couch_gr = obj_lines_to_mesh(include_str!("assets/couch_gr.obj"));
    couch_gr
}

fn table_gr() -> Mesh {
    let table_gr = obj_lines_to_mesh(include_str!("assets/table.obj"));
    table_gr
}

fn mug_gr() -> Mesh {
    let mug_gr = obj_lines_to_mesh(include_str!("assets/mug_gr.obj"));
    mug_gr
}

fn tv_gr() -> Mesh {
    let tv_gr = obj_lines_to_mesh(include_str!("assets/tv_gr.obj"));
    tv_gr
}

// Main room
pub const COUCH_MR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Couch_mr"));
pub const TABLE_MR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Table_mr"));
pub const BLOCK_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Blocks"));
pub const MUGS_MR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Mugs_mr"));

fn couch_mr() -> Mesh {
    let couch_mr = obj_lines_to_mesh(include_str!("assets/couch_mr.obj"));
    couch_mr
}

fn table_mr() -> Mesh {
    let table_mr = obj_lines_to_mesh(include_str!("assets/table_mr.obj"));
    table_mr
}

fn blocks() -> Mesh {
    let blocks = obj_lines_to_mesh(include_str!("assets/block.obj"));
    blocks
}

fn mugs_mr() -> Mesh {
    let mugs_mr = obj_lines_to_mesh(include_str!("assets/mugs_mr.obj"));
    mugs_mr
}

// Bowling room
pub const PINS_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Pins"));
pub const TV_BR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("TV_br"));
pub const SHELF_BR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Shelf_br"));
pub const BALLS_BR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Balls_br"));

fn pins() -> Mesh {
    let pins = obj_lines_to_mesh(include_str!("assets/pins.obj"));
    pins
}

fn tv_br() -> Mesh {
    let tv_br = obj_lines_to_mesh(include_str!("assets/tv_br.obj"));
    tv_br
}

fn shelf_br() -> Mesh {
    let shelf_br = obj_lines_to_mesh(include_str!("assets/shelf_br.obj"));
    shelf_br
}

fn balls_br() -> Mesh {
    let balls_br = obj_lines_to_mesh(include_str!("assets/balls_br.obj"));
    balls_br
}

// Create shaders -- not in use yet
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
        // Galaga room
        io.send(&UploadMesh {
            mesh: avatar(),
            id: AVATAR_RDR,
        });

        io.send(&UploadMesh {
            mesh: couch_gr(),
            id: COUCH_GR_RDR,
        });

        io.send(&UploadMesh {
            mesh: table_gr(),
            id: TABLE_GR_RDR,
        });

        io.send(&UploadMesh {
            mesh: mug_gr(),
            id: MUG_GR_RDR,
        });

        io.send(&UploadMesh {
            mesh: tv_gr(),
            id: TV_GR_RDR,
        });

        // Main room
        io.send(&UploadMesh {
            mesh: couch_mr(),
            id: COUCH_MR_RDR,
        });

        io.send(&UploadMesh {
            mesh: table_mr(),
            id: TABLE_MR_RDR,
        });

        io.send(&UploadMesh {
            mesh: blocks(),
            id: BLOCK_RDR,
        });

        io.send(&UploadMesh {
            mesh: mugs_mr(),
            id: MUGS_MR_RDR,
        });

        // Bowling Room
        io.send(&UploadMesh {
            mesh: pins(),
            id: PINS_RDR,
        });

        io.send(&UploadMesh {
            mesh: tv_br(),
            id: TV_BR_RDR,
        });

        io.send(&UploadMesh {
            mesh: shelf_br(),
            id: SHELF_BR_RDR,
        });

        io.send(&UploadMesh {
            mesh: balls_br(),
            id: BALLS_BR_RDR,
        });

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
        // Declare renders
        // Galaga room
        let avatar_render = Render {
            id: AVATAR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let couch_gr_render = Render {
            id: COUCH_GR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let table_gr_render = Render {
            id: TABLE_GR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let mug_gr_render = Render {
            id: MUG_GR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let tv_gr_render = Render {
            id: TV_GR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        // Main room
        let couch_mr_render = Render {
            id: COUCH_MR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let table_mr_render = Render {
            id: TABLE_MR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let block_render = Render {
            id: BLOCK_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let mugs_mr_render = Render {
            id: MUGS_MR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        // Bowling room
        let pins_render = Render {
            id: PINS_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let tv_br_render = Render {
            id: TV_BR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let shelf_br_render = Render {
            id: SHELF_BR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        let balls_br_render = Render {
            id: BALLS_BR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: None.into(),
        };

        // Create entities
        // Galaga room
        let avatar = io.create_entity().build();
        io.add_component(avatar, Transform::identity());
        io.add_component(avatar, avatar_render);
        io.add_component(avatar, Synchronized);

        let couch_gr = io.create_entity().build();
        io.add_component(couch_gr, Transform::identity());
        io.add_component(couch_gr, couch_gr_render);
        io.add_component(couch_gr, Synchronized);

        let table_gr = io.create_entity().build();
        io.add_component(table_gr, Transform::identity());
        io.add_component(table_gr, table_gr_render);
        io.add_component(table_gr, Synchronized);

        let mug_gr = io.create_entity().build();
        io.add_component(mug_gr, Transform::identity());
        io.add_component(mug_gr, mug_gr_render);
        io.add_component(mug_gr, Synchronized);

        let tv_gr = io.create_entity().build();
        io.add_component(tv_gr, Transform::identity());
        io.add_component(tv_gr, tv_gr_render);
        io.add_component(tv_gr, Synchronized);

        // Main room
        let couch_mr = io.create_entity().build();
        io.add_component(couch_mr, Transform::identity());
        io.add_component(couch_mr, couch_mr_render);
        io.add_component(couch_mr, Synchronized);

        let table_mr = io.create_entity().build();
        io.add_component(table_mr, Transform::identity());
        io.add_component(table_mr, table_mr_render);
        io.add_component(table_mr, Synchronized);

        let blocks = io.create_entity().build();
        io.add_component(blocks, Transform::identity());
        io.add_component(blocks, block_render);
        io.add_component(blocks, Synchronized);

        let mugs_mr = io.create_entity().build();
        io.add_component(mugs_mr, Transform::identity());
        io.add_component(mugs_mr, mugs_mr_render);
        io.add_component(mugs_mr, Synchronized);

        // Bowling room
        let pins = io.create_entity().build();
        io.add_component(pins, Transform::identity());
        io.add_component(pins, pins_render);
        io.add_component(pins, Synchronized);

        let tv_br = io.create_entity().build();
        io.add_component(tv_br, Transform::identity());
        io.add_component(tv_br, tv_br_render);
        io.add_component(tv_br, Synchronized);

        let shelf_br = io.create_entity().build();
        io.add_component(shelf_br, Transform::identity());
        io.add_component(shelf_br, shelf_br_render);
        io.add_component(shelf_br, Synchronized);

        let balls_br = io.create_entity().build();
        io.add_component(balls_br, Transform::identity());
        io.add_component(balls_br, balls_br_render);
        io.add_component(balls_br, Synchronized);

        println!("Hello, server!");
        Self
    }
}

// Defines entry points for the engine to hook into.
// Calls new() for the appropriate state.
make_app_state!(ClientState, ServerState);

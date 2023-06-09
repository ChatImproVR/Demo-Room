use cimvr_common::{
    render::{
        Mesh, MeshHandle, Primitive, Render, ShaderHandle, ShaderSource, UploadMesh,
        DEFAULT_VERTEX_SHADER, RenderExtra, Pos
    },
    Transform,
};
use cimvr_engine_interface::{make_app_state, pkg_namespace, prelude::*, println};
use obj_reader::obj::obj_lines_to_mesh;

mod shaders;
// All state associated with client-side behaviour
struct ClientState;

// Galaga Room
pub const WALLS_GR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Walls_gr"));
pub const WALLS_GR_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Walls_gr"));

pub const AVATAR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Avatar"));
pub const AVATAR_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Avatar"));

pub const COUCH_GR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Couch_gr"));
pub const COUCH_GR_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Couch_gr"));

pub const TABLE_GR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Table_gr"));
pub const TABLE_GR_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Table_gr"));

pub const MUG_GR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Mug_gr"));
pub const TV_GR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("TV_gr"));

fn walls_gr() -> Mesh {
    let walls_gr = obj_lines_to_mesh(include_str!("assets/gr_walls.obj"));
    walls_gr
}

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

// Hallway
pub const WALLS_HALL_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Walls_hall"));
pub const WALLS_HALL_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Walls_hall"));

fn walls_hall() -> Mesh {
    let walls_hall = obj_lines_to_mesh(include_str!("assets/hall_walls.obj"));
    walls_hall
}

// Main room
pub const WALLS_MR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Walls_mr"));
pub const WALLS_MR_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Walls_mr"));

pub const COUCH_MR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Couch_mr"));
pub const TABLE_MR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Table_mr"));
pub const BLOCK_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Blocks"));
pub const MUGS_MR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Mugs_mr"));

fn walls_mr() -> Mesh {
    let walls_mr = obj_lines_to_mesh(include_str!("assets/mr_walls.obj"));
    walls_mr
}

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
pub const WALLS_BR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Walls_br"));
pub const WALLS_BR_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Walls_br"));

pub const PINS_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Pins"));
pub const PINS_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Pins"));

pub const TV_BR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("TV_br"));
pub const SHELF_BR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Shelf_br"));

pub const BALLS_BR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Balls_br"));
pub const BALLS_BR_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Balls_br"));

pub const ALLEY_BR_RDR: MeshHandle = MeshHandle::new(pkg_namespace!("Alley_br"));
pub const ALLEY_BR_SHDR: ShaderHandle = ShaderHandle::new(pkg_namespace!("Alley_br"));

fn walls_br() -> Mesh {
    let walls_br = obj_lines_to_mesh(include_str!("assets/br_walls.obj"));
    walls_br
}

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

fn alley_br() -> Mesh {
    let alley_br = obj_lines_to_mesh(include_str!("assets/br_alley.obj"));
    alley_br
}


impl UserState for ClientState {
    // Implement a constructor
    fn new(io: &mut EngineIo, _sched: &mut EngineSchedule<Self>) -> Self {
        // Galaga room
        io.send(&UploadMesh {
            mesh: walls_gr(),
            id: WALLS_GR_RDR,
        });

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::WALLS_GR_FRAG.to_string(),
            id: WALLS_GR_SHDR,
        });

        io.send(&UploadMesh {
            mesh: avatar(),
            id: AVATAR_RDR,
        });

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::LAV_FRAG.to_string(),
            id: AVATAR_SHDR,
        });

        io.send(&UploadMesh {
            mesh: couch_gr(),
            id: COUCH_GR_RDR,
        });

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::COUCH_FRAG.to_string(),
            id: COUCH_GR_SHDR,
        });

        io.send(&UploadMesh {
            mesh: table_gr(),
            id: TABLE_GR_RDR,
        });

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::TABLE_FRAG.to_string(),
            id: TABLE_GR_SHDR,
        });

        io.send(&UploadMesh {
            mesh: mug_gr(),
            id: MUG_GR_RDR,
        });

        io.send(&UploadMesh {
            mesh: tv_gr(),
            id: TV_GR_RDR,
        });

        // Hallway
        io.send(&UploadMesh {
            mesh: walls_hall(),
            id: WALLS_HALL_RDR,
        });

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::WALLS_HALL_FRAG.to_string(),
            id: WALLS_HALL_SHDR,
        });

        // Main room
        io.send(&UploadMesh {
            mesh: walls_mr(),
            id: WALLS_MR_RDR,
        });

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::WALLS_MR_FRAG.to_string(),
            id: WALLS_MR_SHDR,
        });

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
            mesh: walls_br(),
            id: WALLS_BR_RDR,
        });

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::WALLS_BR_FRAG.to_string(),
            id: WALLS_BR_SHDR,
        });

        io.send(&UploadMesh {
            mesh: pins(),
            id: PINS_RDR,
        });

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::WHITE_FRAG.to_string(),
            id: PINS_SHDR,
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

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::BLACK_FRAG.to_string(),
            id: BALLS_BR_SHDR,
        });

        io.send(&UploadMesh {
            mesh: alley_br(),
            id: ALLEY_BR_RDR,
        });

        io.send(&ShaderSource {
            vertex_src: shaders::GRADIENT_VERT.to_string(),
            fragment_src: shaders::ALLEY_FRAG.to_string(),
            id: ALLEY_BR_SHDR,
        });

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

        let walls_gr_render = Render {
            id: WALLS_GR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(WALLS_GR_SHDR).into(),
        };

        let avatar_render = Render {
            id: AVATAR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(AVATAR_SHDR).into(),
        };

        let (avatar_bb_min, avatar_bb_max) : (Pos, Pos) = avatar().find_bb();
        let avatar_bb_renderextra: RenderExtra = RenderExtra ([
            avatar_bb_min.x, avatar_bb_min.y, avatar_bb_min.z, 0.,
            avatar_bb_max.x, avatar_bb_max.y, avatar_bb_max.z, 0.,
            avatar_bb_max.x - avatar_bb_min.x, avatar_bb_max.y - avatar_bb_min.y, avatar_bb_max.z - avatar_bb_min.z, 0., 
            0., 0., 0., 0.,
        ]);

        let couch_gr_render = Render {
            id: COUCH_GR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(COUCH_GR_SHDR).into(),
        };

        let table_gr_render = Render {
            id: TABLE_GR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(TABLE_GR_SHDR).into(),
        };

        let mug_gr_render = Render {
            id: MUG_GR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(PINS_SHDR).into(),
        };

        let tv_gr_render = Render {
            id: TV_GR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(BALLS_BR_SHDR).into(),
        };

        // Hallway
        let walls_hall_render = Render {
            id: WALLS_HALL_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(WALLS_HALL_SHDR).into(),
        };


        // Main room
        let walls_mr_render = Render {
            id: WALLS_MR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(WALLS_MR_SHDR).into(),
        };

        let couch_mr_render = Render {
            id: COUCH_MR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(COUCH_GR_SHDR).into(),
        };

        let table_mr_render = Render {
            id: TABLE_MR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(TABLE_GR_SHDR).into(),
        };

        let block_render = Render {
            id: BLOCK_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(TABLE_GR_SHDR).into(),
        };

        let mugs_mr_render = Render {
            id: MUGS_MR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(PINS_SHDR).into(),
        };

        // Bowling room
        let walls_br_render = Render {
            id: WALLS_BR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(WALLS_BR_SHDR).into(),
        };

        let pins_render = Render {
            id: PINS_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(PINS_SHDR).into(),
        };

        let tv_br_render = Render {
            id: TV_BR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(BALLS_BR_SHDR).into(),
        };

        let shelf_br_render = Render {
            id: SHELF_BR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(TABLE_GR_SHDR).into(),
        };

        let balls_br_render = Render {
            id: BALLS_BR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(BALLS_BR_SHDR).into(),
        };

        let alley_br_render = Render {
            id: ALLEY_BR_RDR,
            primitive: Primitive::Triangles,
            limit: None.into(),
            shader: Some(ALLEY_BR_SHDR).into(),
        };

        // Create entities
        // Galaga room
        let walls_gr = io.create_entity().build();
        io.add_component(walls_gr, Transform::identity());
        io.add_component(walls_gr, walls_gr_render);
        io.add_component(walls_gr, Synchronized);

        let avatar = io.create_entity().build();
        io.add_component(avatar, Transform::identity());
        io.add_component(avatar, avatar_render);
        io.add_component(avatar, Synchronized);
        // bounding box stuffs
        io.add_component(avatar, avatar_bb_renderextra);

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


        // Hallway
        let walls_hall = io.create_entity().build();
        io.add_component(walls_hall, Transform::identity());
        io.add_component(walls_hall, walls_hall_render);
        io.add_component(walls_hall, Synchronized);

        // Main room
        let walls_mr = io.create_entity().build();
        io.add_component(walls_mr, Transform::identity());
        io.add_component(walls_mr, walls_mr_render);
        io.add_component(walls_mr, Synchronized);

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
        let walls_br = io.create_entity().build();
        io.add_component(walls_br, Transform::identity());
        io.add_component(walls_br, walls_br_render);
        io.add_component(walls_br, Synchronized);

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

        let alley_br = io.create_entity().build();
        io.add_component(alley_br, Transform::identity());
        io.add_component(alley_br, alley_br_render);
        io.add_component(alley_br, Synchronized);

        Self
    }
}

// Defines entry points for the engine to hook into.
// Calls new() for the appropriate state.
make_app_state!(ClientState, ServerState);

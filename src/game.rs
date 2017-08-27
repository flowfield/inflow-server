use rand::{thread_rng, Rng};
use std::sync::mpsc::{Sender, Receiver};

pub type PlayerId = usize;

#[derive(Clone, Debug)]
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Response {
    GameState {nodes: Vec<Node>},
}

#[derive(Clone, Debug)]
pub enum Address {
    Player(PlayerId),
    SomePlayers(Vec<PlayerId>),
    All,
}

#[derive(Clone, Debug)]
pub struct AddressResponse {
    pub whom: Address,
    pub response: Response,
}

#[derive(Clone, Debug)]
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    NewPlayer,
    GetState,
    Restart,
}

#[derive(Clone, Debug)]
pub struct PersonalRequest {
    pub player: PlayerId,
    pub request: Request,
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Debug)]
pub struct Node {
    pos: Point,
    size: f32,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Game {
    pub nodes: Vec<Node>
}

impl Point {
    fn dist(self, other: Point) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
    }
}

impl Node {
    fn new(x: i64, y: i64, size: f32) -> Node {
        Node {pos: Point{x, y}, size}
    }
}

impl Game {
    pub fn new() -> Game {
        Game {nodes: gen_nodes(100)}
    }

    pub fn renew(&mut self) {
        self.nodes = gen_nodes(100);
    }

    pub fn main_loop(mut self,
                     incoming: Receiver<PersonalRequest>,
                     outgoing: Sender<AddressResponse>) {
        loop {
            let p_req = incoming.recv().unwrap();
            let id = p_req.player;
            println!("Game process request: {:?}", p_req);

            let resp = match p_req.request {
                Request::NewPlayer => {
                    AddressResponse {
                        whom: Address::Player(id),
                        response: Response::GameState {nodes: self.nodes.clone()}
                    }
                }
                Request::GetState => {
                    AddressResponse {
                        whom: Address::Player(id),
                        response: Response::GameState {nodes: self.nodes.clone()}
                    }
                }
                Request::Restart => {
                    self.renew();
                    AddressResponse{
                        whom: Address::All,
                        response: Response::GameState {nodes: self.nodes.clone()}
                    }
                }
            };
            println!("Send response: {:?}", resp);
            outgoing.send(resp).unwrap();
        }
    }
}

fn get_nearest_nodes(x: i64, y: i64, nodes: &[Node], n: usize, dist: f32) -> Vec<Node> {
    let mut n = n;

    if n == 0 {
        n = nodes.len()
    }

    let pos = Point{x, y};

    let mut source = nodes.to_vec();
    source.sort_by(|a, b| pos.dist(a.pos).partial_cmp(&pos.dist(b.pos)).unwrap());

    let mut res = vec!();
    for node in &source {
        if dist == 0f32 || pos.dist(node.pos) < dist {
            res.push(node.clone())
        }
        if res.len() >= n {
            break
        }
    }
    res
}

pub fn gen_nodes(n: usize) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec!();
    let mut rng = thread_rng();

    while nodes.len() < n {
        let x = rng.gen_range(-1000, 1000);
        let y = rng.gen_range(-1000, 1000);
        if get_nearest_nodes(x, y, &nodes, 1, 100f32).len() > 0 {
            continue;
        }

        let node = Node::new(x, y, rng.gen_range(0.5, 1.5));
        nodes.push(node)
    }
    nodes
}


use crate::game::enums;
use crate::net;
use async_trait::async_trait;
use rand::Rng;

#[derive(Default)]
pub struct Control {
    // Values are cached to recover the original values,
    // if necessary.
    speed: Option<u8>,
    cached_speed: Option<u8>,
    angle: Option<u16>,
    cached_angle: Option<u16>,
    eject_count: Option<u32>,
    split_count: Option<u32>,
    drop_count: Option<u32>,
}

#[derive(Default)]
pub struct PlayerData<'a> {
    pub name: &'a str,
    pub ticket: &'a str,
    pub skin: Option<enums::Skin>,
    pub rainbow_cycle: Option<enums::ColorCycle>,
    pub name_font: Option<enums::NameFont>,
    pub halo: Option<u8>,
    pub hat: Option<u8>,
    pub particle: Option<u8>,
    pub visibility: Option<enums::PlayerVisibility>,
    pub eject_skin: Option<u8>,
}

#[derive(Default)]
pub struct Net<'a> {
    pub connection_state: Option<enums::ConnectionState>,
    pub server_ip: &'a str,
    pub server_port: u16,
    pub sock: Option<tokio::net::UdpSocket>,

    // Two tokens received from CONNECT_RESULT_2 (0x01).
    // Used to identify the client server-side.
    pub cr2_token1: Option<u32>,
    pub cr2_token2: Option<u32>,

    // Same as above, but randomly generated by ourselves.
    // These also need to be provided sometimes when
    // sending packets, e.g. DISCONNECT (0x07).
    pub rng_token1: u32,
    pub rng_token2: u32,
}

pub struct Bot<'a> {
    pub control: Control,
    pub player_data: PlayerData<'a>,
    pub net: Net<'a>,
}

impl<'a> Bot<'a> {
    pub fn new(name: &'a str, ticket: Option<&'a str>, server_ip: &'a str) -> Self {
        let mut rng = rand::thread_rng();

        Bot {
            control: Control::default(),
            player_data: PlayerData {
                name,
                ticket: ticket.unwrap_or(",-"),
                ..Default::default() // TODO: implement customization
            },
            net: Net {
                server_ip,
                server_port: rng.gen_range(27900..=27901),
                rng_token1: rng.gen::<u32>(),
                rng_token2: rng.gen::<u32>(),
                ..Default::default()
            },
        }
    }
}

#[async_trait]
pub trait UDPClient<'a> {
    async fn udp_tick<'b>(&'b mut self);
    async fn setup_sock(&mut self);
}

#[async_trait]
impl<'a> UDPClient<'a> for Bot<'a> {
    async fn udp_tick<'b>(&'b mut self) {
        // bind to random open port in local machine
        let mut buf = [0; 1024];
        let sock = self.net.sock.as_mut().unwrap();
        let (amt, src) = sock.recv_from(&mut buf).await.unwrap();
        let packet = buf[..amt].to_vec();
        net::redirect(self, packet);
    }

    async fn setup_sock(&mut self) {
        let sock = tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap();
        self.net.sock = Some(sock);
    }
}

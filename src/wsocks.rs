// Redefine a SOCKET Type for c++ fanciness, totally useless
pub type SOCKET = usize;

extern "system"
{
    fn socket(addr_family: usize, sck_type: usize, protocol: usize) -> usize; 
    fn WSAStartup(wVersionRequested: u16, lpWSAData: *const usize) -> usize;
    fn setsockopt(socket: usize, level: usize, optname: usize, optval: *const usize, optlen: usize) -> usize;
    fn send(socket: usize, buffer: *const u8, len: usize, flags: usize) -> usize;
    fn sendto(socket: usize, buffer: *const u8, len: usize, flags: usize, to: *const usize, tolen: usize) -> usize;
    fn closesocket(socket: usize) -> usize;
    fn WSACleanup() -> usize;
    fn WSAGetLastError() -> usize;
    fn inet_pton(addr_family: usize, pszAddrString: *const u8, pAddrBuf: *const usize) -> usize;
}

#[repr(C)]
pub struct WSAData
{
    w_version:          u16,
    w_high_version:     u16,
    sz_description:     [u8;257],
    sz_system_status:   [u8;129],
    i_max_sockets:      u16,
    i_max_udp_dg:       u16,
    lp_vendor_info:     *const u8,
}

impl Default for WSAData
{
    fn default() -> Self
    {
        WSAData
        {
            w_version: 0,
            w_high_version: 0,
            sz_description: [0u8;257],
            sz_system_status: [0u8;129],
            i_max_sockets: 0,
            i_max_udp_dg: 0,
            lp_vendor_info: 0x0 as *const u8,
        }
    }
}

#[repr(C)]
pub struct sockaddr
{
    pub sa_family:  u16,
    pub sa_data:    [u8;14],
}

impl Default for sockaddr
{
    fn default() -> Self
    {
        sockaddr
        {
            sa_family: 0,
            sa_data: [0u8;14],
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct sockaddr_in
{
    pub sin_family: i16,
    pub sin_port:   u16,
    pub sin_addr:   [u8;32],
    pub sin_zero:   [u8;8],
}

impl Default for sockaddr_in
{
    fn default() -> Self
    {
        sockaddr_in
        {
            sin_family: 0,
            sin_port: 0,
            sin_addr: [0u8;32],
            sin_zero: [0u8;8],
        }
    }
}


//Exposed wrappers
pub fn get_socket(addr_family: usize, socket_type: usize, protocol: usize) -> usize
{
    unsafe { socket(addr_family, socket_type, protocol) }
}

pub fn wsa_startup(w_version_required: u16, wsa_data_ptr: *const usize) -> usize
{
    unsafe { WSAStartup(w_version_required, wsa_data_ptr) }
}

pub fn set_sock_opt(socket: usize, level: usize, optname: usize, optval: *const usize, optlen: usize) -> usize
{
    unsafe { setsockopt(socket, level, optname, optval, optlen) }
}

pub fn sock_send(socket: usize, buffer: *const u8, len: usize, flags: usize) -> usize
{
    unsafe { send(socket, buffer, len, flags) }
}

pub fn sock_send_to(socket: usize, buffer: *const u8, len: usize, flags: usize, to: *const usize, tolen: usize) -> usize
{
    unsafe { sendto(socket, buffer, len, flags, to, tolen) }
}

pub fn close_socket(socket: usize) -> usize
{
    unsafe { closesocket(socket) }
}

pub fn wsa_cleanup() -> usize
{
    unsafe { WSACleanup() }
}

pub fn wsa_get_last_error() -> usize
{
    unsafe { WSAGetLastError() }
}

pub fn inet_from_str(addr_family: usize, addr: *const u8, sin_addr_struct: *const usize) -> usize
{
    unsafe { inet_pton(addr_family, addr, sin_addr_struct) }
}


/*
 *
 * Winsock constant
 * extracted from winsock2.h
 * 
 */ 

pub const INVALID_SOCKET:   usize = usize::MAX;

// Socket level option
pub const SOL_SOCKET:       usize = 0xFFFF;

// Socket Options (directly from winsock2.h)
pub const SO_DEBUG:         usize = 0x0001;                     /* turn on debugging info recording */
pub const SO_ACCEPTCONN:    usize = 0x0002;                     /* socket has had listen() */
pub const SO_REUSEADDR:     usize = 0x0004;                     /* allow local address reuse */
pub const SO_KEEPALIVE:     usize = 0x0008;                     /* keep connections alive */
pub const SO_DONTROUTE:     usize = 0x0010;                     /* just use interface addresses */
pub const SO_BROADCAST:     usize = 0x0020;                     /* permit sending of broadcast msgs */
pub const SO_USELOOPBACK:   usize = 0x0040;                     /* bypass hardware when possible */
pub const SO_LINGER:        usize = 0x0080;                     /* linger on close if data present */
pub const SO_OOBINLINE:     usize = 0x0100;                     /* leave received OOB data in line */

pub const SO_DONTLINGER:    usize = !SO_LINGER as usize;
pub const SO_EXCLUSIVEADDRUSE: usize = !SO_REUSEADDR as usize;  /* disallow local address reuse */

/*
 * Additional options.
 */
pub const SO_SNDBUF:        usize = 0x1001;          /* send buffer size */
pub const SO_RCVBUF:        usize = 0x1002;          /* receive buffer size */
pub const SO_SNDLOWAT:      usize = 0x1003;          /* send low-water mark */
pub const SO_RCVLOWAT:      usize = 0x1004;          /* receive low-water mark */
pub const SO_SNDTIMEO:      usize = 0x1005;          /* send timeout */
pub const SO_RCVTIMEO:      usize = 0x1006;          /* receive timeout */
pub const SO_ERROR:         usize = 0x1007;          /* get error status and clear */
pub const SO_TYPE:          usize = 0x1008;          /* get socket type */

/*
 * WinSock 2 extension -- new options
 */
pub const SO_GROUP_ID:      usize = 0x2001;         /* ID of a socket group */
pub const SO_GROUP_PRIORITY: usize = 0x2002;        /* the relative priority within a group*/
pub const SO_MAX_MSG_SIZE:  usize = 0x2003;         /* maximum message size */
pub const SO_PROTOCOL_INFOA: usize = 0x2004;        /* WSAPROTOCOL_INFOA structure */
pub const SO_PROTOCOL_INFOW: usize = 0x2005;        /* WSAPROTOCOL_INFOW structure */


pub const PVD_CONFIG:       usize = 0x3001;         /* configuration info for service provider */
pub const SO_CONDITIONAL_ACCEPT: usize = 0x3002;    /* enable true conditional accept: */
                                                    /*  connection is not ack-ed to the */
                                                    /*  other side until conditional */
                                                    /*  function returns CF_ACCEPT */

// Address Family constants
pub const AF_UNSPEC:        usize = 0;
pub const AF_INET:          usize = 2;
pub const AF_IPX:           usize = 6;
pub const AF_APPELTALK:     usize = 16;
pub const AF_NETBIOS:       usize = 17;
pub const AF_INET6:         usize = 23;
pub const AF_IRDA:          usize = 26;
pub const AF_BTH:           usize = 32;

// Type constants
pub const SOCK_STREAM:      usize = 1;
pub const SOCK_DGRAM:       usize = 2;
pub const SOCK_RAW:         usize = 3;
pub const SOCK_RDM:         usize = 4;
pub const SOCK_SEQPACKET:   usize = 5;

// Protocol constants
pub const NOPROTO:          usize = 0;
pub const IPPROTO_ICMP:     usize = 1;
pub const IPPROTO_IGMP:     usize = 2;
pub const BTHPROTO_RFCOMM:  usize = 3;
pub const IPPROTO_TCP:      usize = 6;
pub const IPPROTO_UDP:      usize = 17;
pub const IPPROTO_ICMPV6:   usize = 58;
pub const IPPROTO_RM:       usize = 113;

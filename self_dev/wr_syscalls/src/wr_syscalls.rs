use core::mem::size_of;
use std::net::{SocketAddr, SocketAddrV4};

use syscalls::{syscall, Sysno};


/// /// /// /// 
/// open
/// /// /// /// 

pub unsafe fn sys_open(file_ptr: *const u8, flags: usize) -> Result<usize, syscalls::Errno> {
    syscall!(Sysno::open, file_ptr, flags)
}

pub unsafe fn sys_openat(dirfd: usize, file_ptr:*const u8, flags: usize) -> Result<usize, syscalls::Errno> {    
    syscall!(Sysno::openat, dirfd, file_ptr, flags)
}


/// /// /// /// 
/// read
/// /// /// /// 


pub unsafe fn sys_creat(file_ptr: *const u8) -> Result<usize, syscalls::Errno> {    
    syscall!(Sysno::creat, file_ptr, 0)
}

pub unsafe fn sys_read(fd: usize, buf_ptr: *const u8, size:usize) -> Result<usize, syscalls::Errno> {
    syscall!(Sysno::read, fd, buf_ptr, size)
}

pub unsafe fn sys_pread(fd: usize, buf_ptr: *const u8, size:usize, offset:usize) -> Result<usize, syscalls::Errno> {
    syscall!(Sysno::pread64, fd, buf_ptr, size, offset)
}

pub unsafe fn sys_readv(fd: usize, iov: *const u8, iovcnt:usize) -> Result<usize, syscalls::Errno> {
    syscall!(Sysno::readv, fd, iov, iovcnt)
}

pub unsafe fn sys_preadv(fd: usize, iov: *const u8, iovcnt:usize, offset: usize) -> Result<usize, syscalls::Errno> {
    syscall!(Sysno::preadv, fd, iov, iovcnt, offset)
}


/// /// /// /// 
/// write
/// /// /// /// 

pub unsafe fn sys_write(fd: usize, buf_ptr: *const u8, size:usize) -> Result<usize, syscalls::Errno> {
    syscall!(Sysno::write, fd, buf_ptr, size)
}

pub unsafe fn sys_pwrite(fd: usize, buf_ptr: *const u8, size:usize, offset: usize) -> Result<usize, syscalls::Errno> {
    syscall!(Sysno::pwrite64, fd, buf_ptr, size, offset)
}

pub unsafe fn sys_writev(fd: usize, iov: *const u8, iovcnt:usize) -> Result<usize, syscalls::Errno> {
    syscall!(Sysno::writev, fd, iov, iovcnt)
}

pub unsafe fn sys_pwritev(fd: usize, iov: *const u8, iovcnt:usize, offset:usize) -> Result<usize, syscalls::Errno> {
    syscall!(Sysno::pwritev, fd, iov, iovcnt, offset)
}

pub unsafe fn sys_sendfile(out_fd: usize, in_fd: usize, offset:*const u8, count: usize)-> Result<usize, syscalls::Errno> {
    syscall!(Sysno::sendfile, out_fd, in_fd, offset, count)
}


/// /// /// /// 
/// close
/// /// /// /// 

pub unsafe fn sys_close(fd: usize)-> Result<usize, syscalls::Errno> {
    syscall!(Sysno::close, fd)
}


/// /// /// /// 
/// process
/// /// /// ///

pub unsafe fn sys_fork()-> Result<usize, syscalls::Errno> {
    syscall!(Sysno::fork)
}

pub unsafe fn sys_execve(pathname: *const u8, argv: *const u8,
    envp: *const u8)-> Result<usize, syscalls::Errno> {
    syscall!(Sysno::execve, pathname, argv, envp)
}


/// /// /// /// 
// net
/// /// /// ///


// #define SOCK_STREAM	1		 stream (connection) socket	
// #define SOCK_DGRAM	2		 datagram (conn.less) socket	
// #define SOCK_RAW	3		 raw socket			
// #define SOCK_RDM	4		 reliably-delivered message	
// #define SOCK_SEQPACKET	5		 sequential packet socket	
// #define SOCK_PACKET	10		 linux specific way of	


// #define AF_UNSPEC	0
// #define AF_UNIX		1	 Unix domain sockets 		
// #define AF_INET		2	 Internet IP Protocol 	
// #define AF_AX25		3	 Amateur Radio AX.25 		
// #define AF_IPX		4	 Novell IPX 			
// #define AF_APPLETALK	5	 Appletalk DDP 		
// #define	AF_NETROM	6	 Amateur radio NetROM 	
// #define AF_BRIDGE	7	 Multiprotocol bridge 	
// #define AF_AAL5		8	 Reserved for Werner's ATM 	
// #define AF_X25		9	 Reserved for X.25 project 	
// #define AF_INET6	10	 IP version 6			
// #define AF_MAX		12	 For now.. 

// pub struct InAddr{

// }

pub struct SockAddrIn{
    pub sa_familty: u8,
    pub port: u8,
    pub addr: SocketAddrV4,
    pub zero: [u8; 8]
}

pub unsafe fn sys_socket(tp:usize)-> Result<usize, syscalls::Errno> {
    syscall!(Sysno::socket, 2, tp, 0)
}

pub unsafe fn sys_bind(socket:usize, sockaddr_in: *const SockAddrIn)-> Result<usize, syscalls::Errno> {
    syscall!(Sysno::bind, socket, sockaddr_in, size_of::<SockAddrIn>())
}
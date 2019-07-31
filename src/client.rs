
mod sendmmsg;

fn main() {
    unsafe {
        sendmmsg::run();
    }
}

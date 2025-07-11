use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::Cancellable;
use gtk::gio::MemoryInputStream;
use gtk::glib::Bytes;
use gtk::glib::Error;
use gtk::Image;

pub fn build_img_from_static_bytes(bytes: &'static [u8]) -> Result<Image, Error> {
    let glib_bytes = Bytes::from_static(bytes);
    let stream = MemoryInputStream::from_bytes(&glib_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE)?;
    let image = Image::from_pixbuf(Some(&pixbuf));

    Ok(image)
}

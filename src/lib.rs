#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn name() {
    unsafe fn print_foto_info() {
        flickcurl_init(); /* optional static initialising of resources */
        let fc = flickcurl_new();

        /* Set configuration, or more likely read from a config file */
        flickcurl_set_oauth_client_key(fc, std::ffi::CString::new("...").unwrap().into_raw());
        flickcurl_set_oauth_client_secret(fc, std::ffi::CString::new("...").unwrap().into_raw());
        flickcurl_set_oauth_token(fc, std::ffi::CString::new("...").unwrap().into_raw());
        flickcurl_set_oauth_token_secret(fc, std::ffi::CString::new("...").unwrap().into_raw());

        let photo: *mut flickcurl_photo = flickcurl_photos_getInfo(fc,
                                                                   std::ffi::CString::new("123456789").unwrap().into_raw()); /* photo ID */

        for field_type in 0..(flickcurl_photo_field_type_PHOTO_FIELD_LAST as u64) {
            /*let datatype: flickcurl_field_value_type = (*photo).fields[field_type].type_;

            match datatype {
                VALUE_TYPE_NONE => print!("none"),
                _ => {
                    println!("val");
                    /*println!("field {} ({}) with {} value: '{}' / {}",
                             flickcurl_get_photo_field_label(field_type),
                             field_type as i32,
                             flickcurl_get_field_value_type_label(datatype),
                             (*photo).fields[field_type].string,
                             (*photo).fields[field_type].integer)*/
                }
            } */
            println!("FIELD!");
        }

        for i in 0..((*photo).tags_count as u8) {
            //let tag: *const flickcurl_tag = (*photo).tags[i];
            println!("tag!");
            /*println!("{}) {} tag: id {} author ID {} name {} raw '{}' cooked '{}' count {}",
                     i,
                     (*tag).machine_tag,
                     (*tag).id,
                     (*tag).author,
                     (*tag).authorname,
                     (*tag).raw,
                     (*tag).cooked,
                     (*tag).count);*/
        }

        flickcurl_free_photo(photo);
        flickcurl_free(fc);
        flickcurl_finish(); /* optional static free of resources */
    }
}

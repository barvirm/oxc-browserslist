use anyhow::Result;
use quote::quote;

use super::{generate_file, generate_rkyv, Caniuse};

pub fn build_caniuse_global_usage(data: &Caniuse) -> Result<()> {
    let global_usage = {
        let mut global_usage: Vec<_> = data
            .agents
            .iter()
            .flat_map(|(name, agent)| {
                agent
                    .usage_global
                    .iter()
                    .map(|(version, usage)| (name.clone(), version.clone(), usage.clone()))
            })
            .collect();

        global_usage.sort_unstable_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());
        global_usage
    };

    generate_rkyv::<_, 256>("caniuse_global_usage.rkyv", global_usage);

    let output = quote! {
        use std::sync::OnceLock;
        use rkyv::vec::ArchivedVec;
        use rkyv::string::ArchivedString;

        type Data = Vec<(String, String, f32)>;
        type ArchivedData = ArchivedVec<(ArchivedString, ArchivedString, f32)>;

        const RKYV_BYTES: &'static [u8] = {
            #[repr(C)]
            struct Aligned<T: ?Sized> {
                _align: [usize; 0],
                bytes: T,
            }
            const ALIGNED: &'static Aligned<[u8]> =
                &Aligned { _align: [], bytes: *include_bytes!("caniuse_global_usage.rkyv") };
            &ALIGNED.bytes
        };

        pub fn caniuse_global_usage() -> &'static ArchivedData {
            static CANIUSE_GLOBAL_USAGE: OnceLock<&ArchivedData> = OnceLock::new();
            CANIUSE_GLOBAL_USAGE.get_or_init(|| {
                #[allow(unsafe_code)]
                unsafe { rkyv::archived_root::<Data>(RKYV_BYTES) }
            })
        }
    };

    generate_file("caniuse_global_usage.rs", output);

    Ok(())
}

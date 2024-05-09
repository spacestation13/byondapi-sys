use std::io::Write;

pub struct Bind {
    pub proc_path: &'static str,
    pub func_name: &'static str,
    pub func_arguments: &'static str,
    pub is_variadic: bool,
}

inventory::collect!(Bind);

pub fn generate_bindings(libname: &str) {
    _ = std::fs::remove_file("./bindings.dm");
    let mut file = std::fs::File::create("./bindings.dm").unwrap();
    let libname_upper = libname.to_uppercase();
    file.write_fmt(format_args!(
        "//THIS FILE IS AUTOMATICALLY GENERATED BY {libname_upper}, PLEASE DO NOT TOUCH IT
//PROC DEFINITIONS MAY MOVE AROUND, THIS IS NORMAL

/* This comment bypasses grep checks */ /var/__{libname}

/proc/__detect_{libname}()
	if (world.system_type == UNIX)
		return __{libname} = \"lib{libname}\"
	else
		return __{libname} = \"{libname}\"

#define {libname_upper} (__{libname} || __detect_{libname}())
    
"
    ))
    .unwrap();
    for thing in inventory::iter::<Bind> {
        let path = thing.proc_path;
        let func_name = thing.func_name;
        let func_arguments = thing.func_arguments;
        let func_arguments_srcless = func_arguments
            .to_owned()
            .replace("src, ", "")
            .replace("src", "");
        if thing.is_variadic {
            //can't directly modify args, fuck you byond
            file.write_fmt(format_args!(
                r#"{path}(...)
	var/list/args_copy = args.Copy()
	args_copy.Insert(1, src)
	return call_ext({libname_upper}, "byond:{func_name}")(arglist(args_copy))

"#
            ))
            .unwrap()
        } else {
            file.write_fmt(format_args!(
                r#"{path}({func_arguments_srcless})
	return call_ext({libname_upper}, "byond:{func_name}")({func_arguments})

"#
            ))
            .unwrap()
        }
    }
}
//! Bundles of the modules in `tests/bundle/`, from the package root cargo tests run in.

#[test]
fn imports_inline_into_one_script() {
    insta::assert_snapshot!(taba_build::bundle("tests/bundle/main.ts").unwrap(), @r"
    (function() {
    	//#region tests/bundle/greeting.ts
    	function greeting(name) {
    		return `hello, ${name}`;
    	}
    	//#endregion
    	//#region tests/bundle/main.ts
    	console.log(greeting(_taba.name));
    	//#endregion
    })();
    ");
}

#[test]
fn unresolved_import_fails() {
    insta::assert_snapshot!(taba_build::bundle("tests/bundle/unresolved.ts").unwrap_err(), @r"
    failed bundle of tests/bundle/unresolved.ts:
    Could not resolve './missing.ts' in tests/bundle/unresolved.ts
    ");
}

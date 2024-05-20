const{ design_data } = wasm_bindgen;

async function addDesign(name) {
	const data = design_data(name);
	const div = document.createElement("div");
	const editor = new JSONEditor(div, {
		"schema": data.schema,
		"disable_edit_json": true,
		"disable_properties": true
	});
	const draw_button = document.createElement("button");
	draw_button.appendChild(document.createTextNode("Draw"));
	div.appendChild(draw_button);
	div.appendChild(document.createElement("br"));
	const canvas = document.createElement("canvas");
	div.appendChild(canvas);
	draw_button.addEventListener("click", () => {
		params = editor.getValue();
		data.draw(canvas, params);
	});
	const idname = data.name.replaceAll(" ","-");
	div.setAttribute("id",idname);
	const form = document.getElementById("form");
	const menu = document.getElementById("menu");
	form.appendChild(div);
	const li = document.createElement("li");
	const a = document.createElement("a");
	a.setAttribute("href","#"+idname);
	a.appendChild(document.createTextNode(data.name));
	li.appendChild(a);
	menu.appendChild(li);
}

async function run() {
	await wasm_bindgen('./symart_wasm_bg.wasm')
	console.log(document.readyState);
	await addDesign("Lines");
	await addDesign("Quasitrap");
	await addDesign("Squiggles");
	$("#form").tabs();
}

run();





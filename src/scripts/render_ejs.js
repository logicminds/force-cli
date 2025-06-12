// render_ejs.js
import fs from 'fs';
import { render } from 'ejs';

const [,, inputPath, outputPath, varsPath] = process.argv;

if (!inputPath || !outputPath || !varsPath) {
  console.error('Usage: node render_ejs.js <template> <output> <vars.json>');
  process.exit(1);
}

const templateStr = fs.readFileSync(inputPath, 'utf8');
const vars = JSON.parse(fs.readFileSync(varsPath, 'utf8'));

const rendered = render(templateStr, vars);
fs.writeFileSync(outputPath, rendered, 'utf8');
console.log(`Rendered EJS template to ${outputPath}`);

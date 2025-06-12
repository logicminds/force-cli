// render_hbs.js
import fs from 'fs';

let handlebars;
try {
  handlebars = await import('handlebars');
  handlebars = handlebars.default || handlebars;
} catch (e) {
  console.error('Handlebars is not installed. Please run: npm install handlebars');
  process.exit(1);
}

const [,, inputPath, outputPath, varsPath] = process.argv;

if (!inputPath || !outputPath || !varsPath) {
  console.error('Usage: node render_hbs.js <template> <output> <vars.json>');
  process.exit(1);
}

const templateStr = fs.readFileSync(inputPath, 'utf8');
const vars = JSON.parse(fs.readFileSync(varsPath, 'utf8'));

const template = handlebars.compile(templateStr);
const rendered = template(vars);

fs.writeFileSync(outputPath, rendered, 'utf8');
console.log(`Rendered Handlebars template to ${outputPath}`);

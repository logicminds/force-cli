# render_jinja.py
import sys
import json
try:
    from jinja2 import Environment, FileSystemLoader
except ImportError:
    print("Jinja2 is not installed. Please install it using 'pip install jinja2'")
    sys.exit(1)

if len(sys.argv) != 4:
    print("Usage: python render_jinja.py <template> <output> <vars.json>")
    sys.exit(1)

template_path, output_path, vars_path = sys.argv[1:4]

with open(template_path, 'r') as f:
    template_content = f.read()

with open(vars_path, 'r') as f:
    variables = json.load(f)

# Create an in-memory Jinja2 environment
env = Environment()
template = env.from_string(template_content)
rendered = template.render(**variables)

with open(output_path, 'w') as f:
    f.write(rendered)

print(f"Rendered Jinja2 template to {output_path}")

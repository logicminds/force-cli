# render_erb.rb
require 'erb'
require 'json'

if ARGV.length != 3
  STDERR.puts "Usage: ruby render_erb.rb <template> <output> <vars.json>"
  exit 1
end

template_path, output_path, vars_path = ARGV

begin
  template_str = File.read(template_path)
rescue => e
  STDERR.puts "Failed to read template: #{e}"
  exit 1
end

begin
  vars = JSON.parse(File.read(vars_path))
rescue => e
  STDERR.puts "Failed to parse variables JSON: #{e}"
  exit 1
end

begin
  renderer = ERB.new(template_str)
  context = OpenStruct.new(vars).instance_eval { binding }
  result = renderer.result(context)
  File.write(output_path, result)
  puts "Rendered ERB template to #{output_path}"
rescue LoadError => e
  STDERR.puts "Missing required Ruby library: #{e}"
  exit 1
rescue => e
  STDERR.puts "Rendering failed: #{e}"
  exit 1
end

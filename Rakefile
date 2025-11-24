require 'json'
require 'pathname'
require 'yaml'

INPUTS = Dir.glob("etc/**/*.yaml").sort

task default: %w[readme:enclosures readme:controllers readme:engines]

namespace :readme do
  task enclosures: INPUTS do |t|
    puts
    puts "#### eGPU Enclosures"
    puts
    puts "Vendor | Model | Year"
    puts ":----- | :---- | :---"
    rows = YAML.load_file("etc/usb/enclosures.yaml", symbolize_names: true)
    rows.each do |row|
      vendor, product = row[:label].split(' ', 2)
      product, year = product.split(' (', 2)
      puts [vendor, product, year.gsub(')', '')].join(" | ")
    end
  end

  task controllers: INPUTS do |t|
    puts
    puts "#### eGPU Controllers"
    puts
    puts "Vendor | Model | Year"
    puts ":----- | :---- | :---"
    rows = YAML.load_file("etc/pci/controllers.yaml", symbolize_names: true)
    rows.each do |row|
      vendor, product = row[:label].split(' ', 2)
      product, year = product.split(' (', 2)
      puts [vendor, product, year.gsub(')', '')].join(" | ")
    end
  end

  task engines: INPUTS do |t|
    puts
    puts "#### eGPU Engines"
    puts
    puts "Vendor | Model | Year"
    puts ":----- | :---- | :---"
    rows = YAML.load_file("etc/pci/engines.yaml", symbolize_names: true)
    rows.each do |row|
      vendor, product = row[:label].split(' ', 2)
      product, year = product.split(' (', 2)
      puts [vendor, product, year.gsub(')', '')].join(" | ")
    end
  end
end

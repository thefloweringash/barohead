#!/usr/bin/env ruby

require 'nokogiri'
require 'json'
require 'json/add/core'

module CanJson
  def to_json(...)
    as_json.to_json(...)
  end
end

RequiredItem = Data.define(:item, :amount, :condition) do
  include CanJson

  def as_json(*)
    { 'item' => item, 'amount' => amount, 'condition' => condition }
  end
end

ProducedItem = Data.define(:id, :amount, :mincondition) do
  include CanJson

  def as_json(*)
    { 'id' => id, 'amount' => amount, 'mincondition' => mincondition }
  end
end

Fabricate = Data.define(
  :suitable_fabricators, :time, :required_items, :required_skills,
  :requires_recipe, :out_condition, :amount, :recycle
) do
  include CanJson

  def as_json(*)
    {
      'suitable_fabricators' => suitable_fabricators,
      'time' => time,
      'required_items' => required_items,
      'required_skills' => required_skills,
      'requires_recipe' => requires_recipe,
      'out_condition' => out_condition,
      'amount' => amount,
      'recycle' => recycle,
    }
  end
end

Deconstruct = Data.define(:time, :required_items, :required_skills, :items) do
  include CanJson

  def as_json(*)
    {
      'time' => time,
      'required_items' => required_items,
      'required_skills' => required_skills,
      'items' => items,
    }
  end
end

ItemRef =  Data.define(:type, :value) do
  include CanJson

  def self.id(value)
    new(:id, value)
  end

  def self.tag(value)
    new(:tag, value)
  end

  def as_json(*)
    { type => value }
  end

  def id
    raise 'Fetch id of non-id ref' unless id?
    value
  end

  def id? = type === :id
  def tag? = type === :tag
end

Item = Struct.new('Item', :id, :nameidentifier, :fabricate, :deconstruct) do
  def initialize(...)
    super
    self.fabricate ||= []
    self.deconstruct ||= []
  end

  def as_json(*)
    {
      'id' => id ,
      'nameidentifier' => nameidentifier,
      'fabricate' => fabricate,
      'deconstruct' => deconstruct,
    }
  end

  def interesting?
    # !fabricate.empty? || !deconstruct.empty?
    true # everything is interesting!
  end
end

# TODO: Overriding Range#to_json is terrible
class Range
  def as_json
    h = {}
    h['min'] = self.begin unless self.begin.nil?
    h['max'] = self.end unless self.end.nil?
    h
  end
end

class ItemDB
  DEFAULT_DE_TIME = 1 # TODO: what does the game do?
  DEFAULT_FAB_TIME = 1 # TODO: what does the game do?
  DEFAULT_FAB_AMOUNT = 1 # TODO: what does the game do?

  attr_reader :items
  attr_reader :texts

  def initialize
    @items = {}
    @texts = {}
  end

  def parse_items(file)
    File.open(file) do |f|
      doc = Nokogiri::XML(f)

      # For a given item, grab all of its construction and deconstruction recipes.

      doc.xpath('/Items/*').each do |item_node|
        unless item_node.has_attribute?('identifier')
          warn "Missing id: #{item_node["name"]}"
          next
        end

        item_id = require_string(item_node, 'identifier')
        warn "Adding item id: #{item_id}"

        nameidentifier = item_node['nameidentifier']

        item = Item.new(id: item_id, nameidentifier:)
        items[item_id] = item

        item_node.xpath('Deconstruct').each do |deconstruct_node|
          time = parse_float(deconstruct_node, 'time', DEFAULT_DE_TIME)
          parse_recipe(deconstruct_node, items_are_requirements: false) =>
           { required_skills:, required_items:, items:, }
          item.deconstruct << Deconstruct.new(
            time:,
            required_skills:,
            required_items:,
            items:,
          )
        end

        item_node.xpath('Fabricate').each do |fabricate_node|
          time = parse_float(fabricate_node, 'requiredtime', DEFAULT_FAB_TIME)
          suitable_fabricators = parse_comma_array(fabricate_node, 'suitablefabricators')
          requires_recipe = parse_boolean(fabricate_node, 'requiresrecipe', false)
          amount = parse_integer(fabricate_node, 'amount', DEFAULT_FAB_AMOUNT)
          out_condition = parse_float(fabricate_node, 'outcondition', 1.0)
          recycle = if (fabricate_node.has_attribute?('displayname'))
            display_name = fabricate_node['displayname']
            case display_name
            when 'recycleitem'
              true
            when 'OxygenTankEmpty'
              # No idea what this means
              false
            else

              raise "Unexpected displayname: #{display_name}"
            end
          else
            false
          end

          parse_recipe(fabricate_node, items_are_requirements: true) =>
            { required_skills:, required_items: }
          item.fabricate << Fabricate.new(
            time:,
            suitable_fabricators:,
            required_skills:,
            required_items:,
            requires_recipe:,
            amount:,
            out_condition:,
            recycle:,
          )
        end
      end
    end
  end

  def parse_texts(file)
    File.open(file) do |f|
      doc = Nokogiri::XML(f)

      doc.xpath('/infotexts').each do |texts_node|
        language = require_string(texts_node, 'language')
        texts = {} # it's just kv
        texts_node.children.each do |child_node|
          next unless child_node.element?
          next unless child_node.name.start_with?("entityname")

          texts[child_node.name] = child_node.children.to_s
        end

        if self.texts[language]
          self.texts[language].merge!(texts)
        else
          self.texts[language] = texts
        end
      end
    end
  end

  private

  def require_string(node, attribute)
    if node.has_attribute?(attribute)
      str = node[attribute]
      raise "Empty string at expected attribute: '#{attribute}'" if str.empty?
      str
    else
      raise "Missing expected attribute '#{attribute}'"
    end
  end

  def parse_float(node, attribute, default)
    if node.has_attribute?(attribute)
      Float(node[attribute])
    else
      default
    end
  end

  def parse_integer(node, attribute, default)
    if node.has_attribute?(attribute)
      Integer(node[attribute])
    else
      default
    end
  end

  def parse_comma_array(node, attribute)
    if node.has_attribute?(attribute)
      node[attribute].split(',')
    else
      []
    end
  end

  def parse_boolean(node, attribute, default)
    if node.has_attribute?(attribute)
      case node[attribute]
      when 'true'
        true
      when 'false'
        false
      else
        raise "Invalid boolean value"
      end
    else
      default
    end

  end

  def parse_item_ref(item_node, allow_tag:)
    ref = nil
    case
    when item_node.has_attribute?('identifier')
      ref = ItemRef.id(require_string(item_node, 'identifier'))
    when item_node.has_attribute?('tag')
      ref = ItemRef.tag(require_string(item_node, 'tag'))
    when item_node.attributes.length == 0
      # TODO: this appears in the slipsuit recipe, with a comment that
      # suggests it has meaning.
      return nil
    else
      raise "Unknown item reference"
    end

    raise "Unexpected tag" if ref.tag? && !allow_tag

    ref
  end

  def parse_condition_range(item_node)
    min = parse_float(item_node, 'mincondition', nil)
    max = parse_float(item_node, 'maxcondition', nil)
    # Make it more obvious when there's no conditions. An infinite range is
    # correct too, but not really in the right spirit.
    if min || max
      Range.new(min, max)
    else
      nil
    end
  end

  def parse_recipe(node, items_are_requirements:)
    required_skills = {}
    required_items = []
    items = []

    node.children.each do |child_node|
      next unless child_node.element?

      case child_node.name
      when 'Item'
        ref = parse_item_ref(child_node, allow_tag: items_are_requirements)
        amount = parse_integer(child_node, 'amount', 1)

        if items_are_requirements then
          required_items << RequiredItem.new(item: ref, amount:, condition: nil)
        else
          mincondition = parse_float(child_node, 'mincondition', nil)
          items << ProducedItem.new(id: ref.id, amount:, mincondition:)
        end
      when 'RequiredItem'
        ref = parse_item_ref(child_node, allow_tag: false)
        next if ref.nil? # see slipsuit recipe, maybe "variantof" handling

        amount = parse_integer(child_node, 'amount', 1)
        condition = parse_condition_range(child_node)

        required_items << RequiredItem.new(item: ref, amount:, condition:)
      when 'RequiredSkill'
        id = require_string(child_node, 'identifier')
        level = parse_integer(child_node, 'level', nil)

        raise if required_skills.has_key?(id)
        required_skills[id] = level
      else
        raise "Unxpected child: #{child_node.name} #{child_node.inspect}"
      end
    end

    { required_items:, required_skills:, items: }
  end
end

db = ItemDB.new
Dir['../Content/Items/**/*.xml'].each do |path|
  warn "Parsing Items in #{path}"
  db.parse_items(path)
end

# TODO: localisation
Dir['../Content/Texts/English/*.xml'].each do |path|
  warn "Parsing Texts in #{path}"
  db.parse_texts(path)
end

puts ({
  items: db.items.values.select(&:interesting?),
  texts: db.texts,
}).to_json

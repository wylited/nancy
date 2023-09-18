CREATE MIGRATION m14raq3uuk4zig3c7cyqjxknojw77qjerig6tp7ocgfp75vq4h6x6q
    ONTO initial
{
  CREATE FUTURE nonrecursive_access_policies;
  CREATE TYPE default::Categories {
      CREATE REQUIRED PROPERTY names -> array<std::str>;
  };
  CREATE TYPE default::Transaction {
      CREATE REQUIRED PROPERTY category -> std::int64;
      CREATE REQUIRED PROPERTY cleared -> std::bool;
      CREATE REQUIRED PROPERTY credit -> std::float32;
      CREATE REQUIRED PROPERTY date -> std::datetime;
      CREATE REQUIRED PROPERTY debit -> std::float32;
      CREATE PROPERTY notes -> std::str;
      CREATE REQUIRED PROPERTY payee -> std::str;
  };
  CREATE TYPE default::Account {
      CREATE MULTI LINK categories -> default::Categories;
      CREATE MULTI LINK transactions -> default::Transaction;
      CREATE REQUIRED PROPERTY balance -> std::float32;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
};

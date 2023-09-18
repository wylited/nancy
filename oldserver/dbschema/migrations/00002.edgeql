CREATE MIGRATION m1zrwweun5wylvo3zy33kjizsrazs4krvwqpu6qi3y6s3agr5hl4xa
    ONTO m14raq3uuk4zig3c7cyqjxknojw77qjerig6tp7ocgfp75vq4h6x6q
{
  ALTER TYPE default::Categories {
      DROP PROPERTY names;
  };
  ALTER TYPE default::Categories RENAME TO default::Category;
  ALTER TYPE default::Category {
      CREATE REQUIRED PROPERTY name -> std::str {
          SET REQUIRED USING ('unnamed');
      };
  };
  ALTER TYPE default::Transaction RENAME TO default::Transactions;
};

CREATE MIGRATION m16bubsxb7coaw54mt7wxnp2bfntfaaufd6xdc7fcev2unxlu342rq
    ONTO m1zrwweun5wylvo3zy33kjizsrazs4krvwqpu6qi3y6s3agr5hl4xa
{
  CREATE TYPE default::AccountType {
      CREATE REQUIRED PROPERTY name -> std::str;
  };
  ALTER TYPE default::Account {
      CREATE MULTI LINK accounttype -> default::AccountType;
  };
  ALTER TYPE default::Account {
      DROP LINK categories;
  };
  ALTER TYPE default::Transactions {
      DROP PROPERTY category;
  };
  ALTER TYPE default::Transactions {
      CREATE MULTI LINK category -> default::Category;
  };
};

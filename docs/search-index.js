var N = null;var searchIndex = {};
searchIndex["discord_bots"]={"doc":"","items":[[3,"Snowflake","discord_bots","Snowflake wrapper for deserializing snowflake strings from api",N,N],[3,"ParamError","","",N,N],[3,"StatusError","","Error when statuscode != OK",N,N],[3,"SimpleUser","","DBL simple user",N,N],[12,"id","","",0,N],[12,"username","","",0,N],[12,"discriminator","","",0,N],[12,"avatar","","",0,N],[12,"def_avatar","","",0,N],[3,"User","","Dbl user: https://discordbots.org/api/docs#users",N,N],[12,"user","","",1,N],[12,"bio","","",1,N],[12,"banner","","",1,N],[12,"social","","",1,N],[12,"color","","",1,N],[12,"supporter","","",1,N],[12,"certified_dev","","",1,N],[12,"is_mod","","",1,N],[12,"is_web_mod","","",1,N],[12,"admin","","",1,N],[3,"Bot","","Dbl bot: https://discordbots.org/api/docs#bots",N,N],[12,"user","","",2,N],[12,"lib","","",2,N],[12,"prefix","","",2,N],[12,"short_desc","","",2,N],[12,"long_desc","","",2,N],[12,"tags","","",2,N],[12,"website","","",2,N],[12,"support","","",2,N],[12,"github","","",2,N],[12,"owners","","",2,N],[12,"invite","","",2,N],[12,"date","","",2,N],[12,"certified","","",2,N],[12,"vanity","","",2,N],[12,"points","","",2,N],[3,"BotStats","","Dbl bot stats: https://discordbots.org/api/docs#bots",N,N],[12,"server_count","","",3,N],[12,"shards","","",3,N],[12,"shard_count","","",3,N],[3,"PostBotStats","","Use this to post stats:",N,N],[12,"server_count","","",4,N],[12,"shard_id","","",4,N],[12,"shard_count","","",4,N],[3,"Client","","The dbl client",N,N],[12,"token","","",5,N],[12,"client","","",5,N],[3,"CustomizeWidget","","Widget colors: https://discordbots.org/api/docs#widgets",N,N],[12,"color_map","","",6,N],[12,"widget_type","","",6,N],[12,"no_avatar","","",6,N],[3,"BotListing","","Bot listing",N,N],[12,"limit","","",7,N],[12,"offset","","",7,N],[12,"search","","",7,N],[12,"sort","","",7,N],[12,"fields","","",7,N],[3,"BotList","","",N,N],[12,"results","","",8,N],[12,"limit","","",8,N],[12,"offset","","",8,N],[12,"count","","",8,N],[12,"total","","",8,N],[4,"ServerCount","","Server count enum",N,N],[13,"Sharded","","",9,N],[13,"Single","","",9,N],[4,"BotListingSort","","Sorting options for bot listing",N,N],[13,"Points","","",10,N],[13,"MonthlyPoints","","",10,N],[13,"Date","","",10,N],[13,"ServerCount","","",10,N],[13,"Reverse","","",10,N],[7,"API","","Discord bots API root",N,N],[11,"clone","","",11,[[["self"]],["snowflake"]]],[11,"fmt","","",11,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",12,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",13,[[["self"],["formatter"]],["result"]]],[11,"clone","","",0,[[["self"]],["simpleuser"]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"clone","","",1,[[["self"]],["user"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"clone","","",2,[[["self"]],["bot"]]],[11,"fmt","","",2,[[["self"],["formatter"]],["result"]]],[11,"clone","","",3,[[["self"]],["botstats"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"clone","","",9,[[["self"]],["servercount"]]],[11,"fmt","","",9,[[["self"],["formatter"]],["result"]]],[11,"clone","","",4,[[["self"]],["postbotstats"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"new","","Makes a PostBotStats out of a ServerCount. You can set the shard id/count with methods",4,[[["servercount"]],["self"]]],[11,"shard_id","","Set the shard id",4,[[["self"],["usize"]],["self"]]],[11,"shard_count","","Set the shard count",4,[[["self"],["usize"]],["self"]]],[11,"get","","",1,[[["snowflake"]],["result",["error"]]]],[11,"new","","Initialize a new dbl client from a token: https://discordbots.org/api/docs#reference",5,[[["str"]],["self"]]],[11,"get_votes","","Get the last 1k votes from your bot",5,[[["self"]],["result",["vec","error"]]]],[11,"get_voted","","Check if a user has voted",5,[[["self"],["snowflake"]],["result",["bool","error"]]]],[11,"get_bot","","Get bot info",5,[[["self"]],["result",["bot","error"]]]],[11,"get_stats","","Get bot stats",5,[[["self"]],["result",["bot","error"]]]],[11,"post_stats","","Post bot stats",5,[[["self"],["postbotstats"]],["result",["error"]]]],[11,"new","","Makes a new CustomizeWidget When setting colors, make sure not to include the hash",6,[[],["self"]]],[11,"widget_type","","Either owner, status, upvotes, servers, or lib",6,[[["self"],["string"]],["self"]]],[11,"no_avatar","","",6,[[["self"]],["self"]]],[11,"top_color","","",6,[[["self"],["str"]]]],[11,"middle_color","","",6,[[["self"],["str"]]]],[11,"username_color","","",6,[[["self"],["str"]]]],[11,"certified_color","","",6,[[["self"],["str"]]]],[11,"data_color","","",6,[[["self"],["str"]]]],[11,"label_color","","",6,[[["self"],["str"]]]],[11,"highlight_color","","",6,[[["self"],["str"]]]],[11,"avatar_bg","","",6,[[["self"],["str"]]]],[11,"left_color","","",6,[[["self"],["str"]]]],[11,"right_color","","",6,[[["self"],["str"]]]],[11,"left_text_color","","",6,[[["self"],["str"]]]],[11,"right_text_color","","",6,[[["self"],["str"]]]],[11,"get","","Get a bots data by its id",2,[[["snowflake"]],["result",["error"]]]],[11,"get_stats","","Gets a bots stats by id",2,[[["snowflake"]],["result",["botstats","error"]]]],[11,"get_widget","","Get a bot's widget image url `ext` determines the extension, svg or png",2,[[["snowflake"],["str"],["option",["customizewidget"]]],["string"]]],[11,"clone","","",10,[[["self"]],["botlistingsort"]]],[11,"fmt","","",10,[[["self"],["formatter"]],["result"]]],[11,"reverse","","Reverses the sorting order",10,[[["self"]],["self"]]],[11,"to_string","","",10,[[["self"]],["string"]]],[11,"clone","","",7,[[["self"]],["botlisting"]]],[11,"fmt","","",7,[[["self"],["formatter"]],["result"]]],[11,"clone","","",8,[[["self"]],["botlist"]]],[11,"fmt","","",8,[[["self"],["formatter"]],["result"]]],[11,"new","","",7,[[],["self"]]],[11,"search","","",7,[[["self"],["string"]],["self"]]],[11,"sort","","",7,[[["self"],["botlistingsort"]],["self"]]],[11,"limit","","",7,[[["self"],["i64"]],["self"]]],[11,"offset","","",7,[[["self"],["i64"]],["self"]]],[11,"fields","","",7,[[["self"],["str"]],["self"]]],[11,"exec","","Execute the BotListing and get the BotList",7,[[["self"]],["result",["botlist","error"]]]],[11,"into","","",11,[[["self"]],["u64"]]],[11,"from","","",11,[[["u64"]],["self"]]],[11,"fmt","","",11,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",11,[[["d"]],["result",["snowflake"]]]],[11,"from","","",11,[[["t"]],["t"]]],[11,"into","","",11,[[["self"]],["u"]]],[11,"to_owned","","",11,[[["self"]],["t"]]],[11,"clone_into","","",11,N],[11,"to_string","","",11,[[["self"]],["string"]]],[11,"try_from","","",11,[[["u"]],["result"]]],[11,"borrow","","",11,[[["self"]],["t"]]],[11,"borrow_mut","","",11,[[["self"]],["t"]]],[11,"try_into","","",11,[[["self"]],["result"]]],[11,"get_type_id","","",11,[[["self"]],["typeid"]]],[11,"from","","",12,[[["t"]],["t"]]],[11,"into","","",12,[[["self"]],["u"]]],[11,"to_string","","",12,[[["self"]],["string"]]],[11,"try_from","","",12,[[["u"]],["result"]]],[11,"borrow","","",12,[[["self"]],["t"]]],[11,"borrow_mut","","",12,[[["self"]],["t"]]],[11,"try_into","","",12,[[["self"]],["result"]]],[11,"get_type_id","","",12,[[["self"]],["typeid"]]],[11,"as_fail","","",12,[[["self"]],["fail"]]],[11,"from","","",13,[[["t"]],["t"]]],[11,"into","","",13,[[["self"]],["u"]]],[11,"to_string","","",13,[[["self"]],["string"]]],[11,"try_from","","",13,[[["u"]],["result"]]],[11,"borrow","","",13,[[["self"]],["t"]]],[11,"borrow_mut","","",13,[[["self"]],["t"]]],[11,"try_into","","",13,[[["self"]],["result"]]],[11,"get_type_id","","",13,[[["self"]],["typeid"]]],[11,"as_fail","","",13,[[["self"]],["fail"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,N],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,N],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,N],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,N],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"from","","",4,[[["t"]],["t"]]],[11,"into","","",4,[[["self"]],["u"]]],[11,"to_owned","","",4,[[["self"]],["t"]]],[11,"clone_into","","",4,N],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"from","","",5,[[["t"]],["t"]]],[11,"into","","",5,[[["self"]],["u"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"try_into","","",5,[[["self"]],["result"]]],[11,"get_type_id","","",5,[[["self"]],["typeid"]]],[11,"from","","",6,[[["t"]],["t"]]],[11,"into","","",6,[[["self"]],["u"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"try_into","","",6,[[["self"]],["result"]]],[11,"get_type_id","","",6,[[["self"]],["typeid"]]],[11,"from","","",7,[[["t"]],["t"]]],[11,"into","","",7,[[["self"]],["u"]]],[11,"to_owned","","",7,[[["self"]],["t"]]],[11,"clone_into","","",7,N],[11,"try_from","","",7,[[["u"]],["result"]]],[11,"borrow","","",7,[[["self"]],["t"]]],[11,"borrow_mut","","",7,[[["self"]],["t"]]],[11,"try_into","","",7,[[["self"]],["result"]]],[11,"get_type_id","","",7,[[["self"]],["typeid"]]],[11,"from","","",8,[[["t"]],["t"]]],[11,"into","","",8,[[["self"]],["u"]]],[11,"to_owned","","",8,[[["self"]],["t"]]],[11,"clone_into","","",8,N],[11,"try_from","","",8,[[["u"]],["result"]]],[11,"borrow","","",8,[[["self"]],["t"]]],[11,"borrow_mut","","",8,[[["self"]],["t"]]],[11,"try_into","","",8,[[["self"]],["result"]]],[11,"get_type_id","","",8,[[["self"]],["typeid"]]],[11,"from","","",9,[[["t"]],["t"]]],[11,"into","","",9,[[["self"]],["u"]]],[11,"to_owned","","",9,[[["self"]],["t"]]],[11,"clone_into","","",9,N],[11,"try_from","","",9,[[["u"]],["result"]]],[11,"borrow","","",9,[[["self"]],["t"]]],[11,"borrow_mut","","",9,[[["self"]],["t"]]],[11,"try_into","","",9,[[["self"]],["result"]]],[11,"get_type_id","","",9,[[["self"]],["typeid"]]],[11,"from","","",10,[[["t"]],["t"]]],[11,"into","","",10,[[["self"]],["u"]]],[11,"to_owned","","",10,[[["self"]],["t"]]],[11,"clone_into","","",10,N],[11,"to_string","","",10,[[["self"]],["string"]]],[11,"try_from","","",10,[[["u"]],["result"]]],[11,"borrow","","",10,[[["self"]],["t"]]],[11,"borrow_mut","","",10,[[["self"]],["t"]]],[11,"try_into","","",10,[[["self"]],["result"]]],[11,"get_type_id","","",10,[[["self"]],["typeid"]]]],"paths":[[3,"SimpleUser"],[3,"User"],[3,"Bot"],[3,"BotStats"],[3,"PostBotStats"],[3,"Client"],[3,"CustomizeWidget"],[3,"BotListing"],[3,"BotList"],[4,"ServerCount"],[4,"BotListingSort"],[3,"Snowflake"],[3,"ParamError"],[3,"StatusError"]]};
initSearch(searchIndex);

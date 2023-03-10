import interactions
import asyncio
from config import token

bot = interactions.Client(token, intents=interactions.Intents.DEFAULT | interactions.Intents.GUILD_MESSAGE_CONTENT)

dps_queue = []
healer_queue = []
tank_queue = []
prevmsg = ""
lock = asyncio.Lock()


dps_button = interactions.Button(
	style=interactions.ButtonStyle.DANGER,
	label="DPS",
	custom_id="dps_click",
)

tank_button = interactions.Button(
	style=interactions.ButtonStyle.SECONDARY,
	label="Tank",
	custom_id="tank_click",
)

healer_button = interactions.Button(
	style=interactions.ButtonStyle.SUCCESS,
	label="Healer",
	custom_id="healer_click",
)

@bot.event
async def on_start():
	for guild in bot.guilds:
		chans = await guild.get_all_channels()
		for channel in chans:
			if channel.name == "mythic-plus-pickup":
				tank_queue_nicks = []
				dps_queue_nicks = []
				healer_queue_nicks = []
				for tank in tank_queue:
					member = await interactions.get(bot, interactions.Member, object_id=tank.id, guild_id=guild.id)
					if member.nick == None:
						member = member.username
					else:
						member = member.nick
					tank_queue_nicks.append(member)
				for healer in healer_queue:
					member = await interactions.get(bot, interactions.Member, object_id=healer.id, guild_id=guild.id)
					if member.nick == None:
						member = member.username
					else:
						member = member.nick
					healer_queue_nicks.append(member)
				for dps in dps_queue:
					member = await interactions.get(bot, interactions.Member, object_id=dps.id, guild_id=guild.id)
					if member.nick == None:
						member = member.username
					else:
						member = member.nick
					dps_queue_nicks.append(member)
				await channel.send("********Bot has been reloaded********")
				await channel.send("The current queue:" + "\n" +
					"<:tank:444634700523241512> : " + ', '.join(tank_queue_nicks) + "\n" +
					"<:heal:444634700363857921> : " + ', '.join(healer_queue_nicks) + "\n" +
					"<:dps:444634700531630094> : " + ', '.join(dps_queue_nicks) + "\n" +
					"Click a button to add to the queue.",
					components=[tank_button, healer_button, dps_button])



@bot.event
async def on_message_create(message: interactions.message):
	global prevmsg
	chan = await interactions.get(bot, interactions.Channel, object_id=message.channel_id)
	if str(chan) == "mythic-plus-pickup":
		if message.author != bot.me:
			if prevmsg != "":
				tank_queue_nicks = []
				dps_queue_nicks = []
				healer_queue_nicks = []
				for tank in tank_queue:
					member = await interactions.get(bot, interactions.Member, object_id=tank.id, guild_id=message.guild_id)
					if member.nick == None:
						member = member.username
					else:
						member = member.nick
					tank_queue_nicks.append(member)
				for healer in healer_queue:
					member = await interactions.get(bot, interactions.Member, object_id=healer.id, guild_id=message.guild_id)
					if member.nick == None:
						member = member.username
					else:
						member = member.nick
					healer_queue_nicks.append(member)
				for dps in dps_queue:
					member = await interactions.get(bot, interactions.Member, object_id=dps.id, guild_id=message.guild_id)
					if member.nick == None:
						member = member.username
					else:
						member = member.nick
					dps_queue_nicks.append(member)
				await prevmsg.delete()
				await chan.send("The current queue:" + "\n" +
					"<:tank:444634700523241512> : " + ', '.join(tank_queue_nicks) + "\n" +
					"<:heal:444634700363857921> : " + ', '.join(healer_queue_nicks) + "\n" +
					"<:dps:444634700531630094> : " + ', '.join(dps_queue_nicks) + "\n" +
					"Click a button to add to the queue.",
					components=[tank_button, healer_button, dps_button])
		if message.author == bot.me:
			if "Click a button to add to the queue." in message.content:
				prevmsg = message
			else:
				if prevmsg != "":
					tank_queue_nicks = []
					dps_queue_nicks = []
					healer_queue_nicks = []
					for tank in tank_queue:
						member = await interactions.get(bot, interactions.Member, object_id=tank.id, guild_id=message.guild_id)
						if member.nick == None:
							member = member.username
						else:
							member = member.nick
						tank_queue_nicks.append(member)
					for healer in healer_queue:
						member = await interactions.get(bot, interactions.Member, object_id=healer.id, guild_id=message.guild_id)
						if member.nick == None:
							member = member.username
						else:
							member = member.nick
						healer_queue_nicks.append(member)
					for dps in dps_queue:
						member = await interactions.get(bot, interactions.Member, object_id=dps.id, guild_id=message.guild_id)
						if member.nick == None:
							member = member.username
						else:
							member = member.nick
						dps_queue_nicks.append(member)
					await prevmsg.delete()
					await chan.send("The current queue:" + "\n" +
					"<:tank:444634700523241512> : " + ', '.join(tank_queue_nicks) + "\n" +
					"<:heal:444634700363857921> : " + ', '.join(healer_queue_nicks) + "\n" +
					"<:dps:444634700531630094> : " + ', '.join(dps_queue_nicks) + "\n" +
					"Click a button to add to the queue.",
					components=[tank_button, healer_button, dps_button])
			
async def queue_check(ctx: interactions.ComponentContext):
	tank_filled_queue = []
	healer_filled_queue = []
	dps_filled_queue = []
	filled_queue = []
	if len(filled_queue) < 5:
		for tank in tank_queue:
			if len(tank_filled_queue) < 1:
				tank_filled_queue.append(tank)
		for healer in healer_queue:
			if len(healer_filled_queue) < 1:
				if healer not in tank_filled_queue:
					healer_filled_queue.append(healer)
		for dps in dps_queue:
			if dps not in tank_filled_queue or dps not in healer_filled_queue:
				if len(dps_filled_queue) < 3:
					dps_filled_queue.append(dps)
	if len(tank_filled_queue) == 1:
		if len(healer_filled_queue) == 1:
			if len(dps_filled_queue) == 3:
				for tank in tank_filled_queue:
					filled_queue.append(tank)
				for healer in healer_filled_queue:
					filled_queue.append(healer)
				for dps in dps_filled_queue:
					filled_queue.append(dps_filled_queue)
				await ctx.channel.send("A group has been found!" + ' , '.join((f"<@{player.id}>")) for player in filled_queue)
				for user in filled_queue:
					if user in tank_queue:
						tank_queue.remove(user)
					if user in healer_queue:
						healer_queue.remove(user)
					if user in dps_queue:
						dps_queue.remove(user)
                

@bot.component("dps_click")
async def _click_me(ctx: interactions.ComponentContext):
	user=ctx.user
	member = await interactions.get(bot, interactions.Member, object_id=user.id, guild_id=ctx.guild_id)
	if member.nick == None:
		member = member.username
	else:
		member = member.nick
	async with lock: 
		if user not in dps_queue:
			dps_queue.append(user)
			await ctx.channel.send(f"{member} added to queue as dps!")
			await queue_check(ctx)
		else:
			dps_queue.remove(user)
			await ctx.channel.send(f"{member} has been removed from the dps queue.")

@bot.component("tank_click")
async def _click_me(ctx: interactions.ComponentContext):
	user=ctx.user
	member = await interactions.get(bot, interactions.Member, object_id=user.id, guild_id=ctx.guild_id)
	if member.nick == None:
		member = member.username
	else:
		member = member.nick
	async with lock: 
		if user not in tank_queue:
			tank_queue.append(user)
			await ctx.channel.send(f"{member} added to queue as tank!")
			await queue_check(ctx)
		else:
			tank_queue.remove(user)
			await ctx.channel.send(f"{member} has been removed from the tank queue.")

@bot.component("healer_click")
async def _click_me(ctx: interactions.ComponentContext):
	user=ctx.user
	member = await interactions.get(bot, interactions.Member, object_id=user.id, guild_id=ctx.guild_id)
	if member.nick == None:
		member = member.username
	else:
		member = member.nick
	async with lock: 
		if user not in healer_queue:
			healer_queue.append(user)
			await ctx.channel.send(f"{member} added to queue as healer!")
			await queue_check(ctx)
		else:
			healer_queue.remove(user)
			await ctx.channel.send(f"{member} has been removed from the healer queue.")

@bot.command(
	name="add",
	description="Add to the queue.",
	options= [
		interactions.Option(
			name="role",
			description="Roles you want to join as, seperated by comma. Accepted values are: tank, healer, dps.",
			type=interactions.OptionType.STRING,
			required=True,
			autocomplete=True,
		),
	],
)
async def _button(ctx: interactions.CommandContext, role: str = ""):
	user = ctx.user
	member = await interactions.get(bot, interactions.Member, object_id=ctx.user.id, guild_id=ctx.guild_id)
	if member.nick == None:
		member = member.username
	else:
		member = member.nick
	role_add_list = []
	role_remove_list = []
	if role != "":
		if "tank" in role:
			if user not in tank_queue:
				role_add_list.append("Tank")
				tank_queue.append(user)
				await queue_check(ctx)
			else:
				role_remove_list.append("Tank")
				tank_queue.remove(user)

		if "heal" in role:
			if user not in healer_queue:
				role_add_list.append("Healer")
				healer_queue.append(user)
				await queue_check(ctx)
			else:
				role_remove_list.append("Healer")
				healer_queue.remove(user)
		if "dps" in role:
			if user not in dps_queue:
				role_add_list.append("DPS")
				dps_queue.append(user)
				await queue_check(ctx)
			else:
				role_remove_list.append("DPS")
				dps_queue.remove(user)
		if role_add_list != []:
			await ctx.send(f"{member} joined the queue as {', '.join(role_add_list)}.")
		if role_remove_list != []:
			await ctx.send(f"{member} left the queue as {', '.join(role_remove_list)}.")
		if "tank" not in role:
			if "heal" not in role:
				if "dps" not in role:
					await user.send("No valid role specified, acceptable roles are `tank` `healer` and `dps`.")

bot.start()
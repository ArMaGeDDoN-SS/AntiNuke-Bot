![alt text](https://github.com/ArMaGeDDoN-SS/AntiNuke-Bot/blob/main/images/banner.png)

<h1 align="center"> Atomic Security - AntiNuke Bot </h1>

[![Discord](https://img.shields.io/discord/1216704766491623455?style=for-the-badge&logo=discord&label=discord&color=111f63)](https://img.shields.io/discord/1216704766491623455?style=for-the-badge&logo=discord&label=Discord&color=0a154d&link=https%3A%2F%2Fdiscord.gg%2FN3XG9FH2v9)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/ArMaGeDDoN-SS/AntiNuke-Bot?style=for-the-badge&logo=rust&color=631124)


<p align="center" dir="auto">Atomic Security is a discord bot designed to protect the server from nuke bots, as well as unauthorized editing of the server (changing channels, roles, etc.). The bot is easily configured through the configuration file.</p> 

<p align="center" dir="auto">
  <b>NOTE:</b> <i>The bot DOES NOT have any commands, and the configuration option is not finalized: the server_config.json file does not affect the bot's operation. In the future, this will be fixed, and you will be able to configure the bot in more detail: issue any other punishment for unauthorized server changes (except for a ban), for example.</i>
</p>

<h1 align="center" dir="auto"><a id="user-content-disclaimer" class="anchor" aria-hidden="true" href="#disclaimer"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Mechanics of black and white lists</h1>

<ul>
<li><p>Whitelist is a list of bots or members whose server editing actions are not recognized by the bot as dangerous. Please note that Atom will not interfere with them in any way, so only add verified bots and trusted members to the whitelist.</p></li>
<li><p>The blacklist is a list of dangerous bots that Atom removes immediately after they join the server.</p></li>
</ul>

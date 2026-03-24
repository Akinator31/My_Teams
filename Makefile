NAME_SERVER = myteams_server
NAME_CLIENT = myteams_cli

all:
	cargo build --release
	cp target/release/myteams_server ./$(NAME_SERVER)
	cp target/release/myteams_cli ./$(NAME_CLIENT)

clean:
	cargo clean

fclean: clean
	rm -f $(NAME_SERVER)
	rm -f $(NAME_CLIENT)

re: fclean all

.PHONY: all clean fclean re

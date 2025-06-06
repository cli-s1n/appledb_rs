import { styled, Typography } from "@mui/material";
import logo from "../logo.png";
import { Link as RouterLink } from "react-router-dom";
import MuiAppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import { DRAWER_WIDTH, HOME_ROUTE } from "../Constants";
import React from "react";

const AppBar = styled(MuiAppBar, {
  shouldForwardProp: (prop) => prop !== "open",
})(({ theme }) => ({
  transition: theme.transitions.create(["margin", "width"], {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  variants: [
    {
      props: ({ open }) => open,
      style: {
        width: `calc(100% - ${DRAWER_WIDTH}px)`,
        marginLeft: `${DRAWER_WIDTH}px`,
        transition: theme.transitions.create(["margin", "width"], {
          easing: theme.transitions.easing.easeOut,
          duration: theme.transitions.duration.enteringScreen,
        }),
      },
    },
  ],
}));

const CustomAppBar = (props) => {
  const { drawerOpen, handleDrawerOpen } = props;

  return (
    <AppBar
      position="fixed"
      open={drawerOpen}
      sx={{ backgroundColor: "#555555" }}
    >
      <Toolbar>
        <IconButton
          color="inherit"
          aria-label="open drawer"
          onClick={handleDrawerOpen}
          edge="start"
          style={{ backgroundColor: "transparent" }}
          sx={[
            {
              mr: 2,
            },
            drawerOpen && { display: "none" },
          ]}
        >
          <MenuIcon />
        </IconButton>
        <IconButton
          component={RouterLink}
          to={HOME_ROUTE}
          style={{ backgroundColor: "transparent" }}
          sx={{ height: "64px", maxHeight: "64px" }}
        >
          <img
            style={{
              height: "inherit",
              maxHeight: "inherit",
              filter: "grayscale(0.5)",
            }}
            src={logo}
            alt="logo"
          />
        </IconButton>
        <Typography variant="h6" noWrap component="div">
          AppleDB
        </Typography>
      </Toolbar>
    </AppBar>
  );
};

export default CustomAppBar;

# Variables
LFLAGS = -lpthread
ODUMP = objdump
ODFLAGS = -d
OBJCOPY = objcopy
OBJFLAGS = -O binary
CC = clang
AR = ar
CFLAGS = -Werror -Wextra -Wno-unused-command-line-argument -O3 -std=c11

# Directories
OBJDIR = obj
TGTDIR = target
INCDIR = include
SRCDIR = src
TSTDIR = tests
LIBDIR = lib
DIRS = $(OBJDIR) $(TGTDIR) $(INCDIR) $(LIBDIR) 

# Target name
TGT = alang
LLIST = tlp

OBJS := $(foreach LIB,$(LLIST),$(OBJDIR)/$(LIB).o)
LIBS := $(foreach LIB,$(LLIST),$(LIBDIR)/lib$(LIB).a)
TSTO := $(patsubst $(TSTDIR)/%.c,$(OBJDIR)/$(TSTDIR)/%.o, $(wildcard $(TSTDIR)/*.c))
TSTE := $(foreach LIB,$(LLIST),$(TGTDIR)/$(LIB).exe)

.PHONY: all 
all: $(TGTDIR)/$(TGT)

$(TGTDIR)/$(TGT): $(OBJS) | $(LIBS)
	@echo $(LIBS)
	@echo $(OBJS)

$(TGTDIR)/: $(TSTO) | $(OBJS)
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -o $@ $(filter $(OBJDIR)/$(TSTDIR)/$(basename $(notdir $@)).o,$(TSTO)) $(OBJS) $(LFLAGS)

$(LIBDIR)/%.a: $(OBJS)
	$(AR) rcs $@ $(filter $(subst lib,$(OBJDIR)/, $(basename $(notdir $@)))/%.o, $(OBJS))

$(OBJDIR)/$(TSTDIR)/%.o: $(TSTDIR)/%.c
	@mkdir -p $(@D)
	$(CC) $(CPPFLAGS) $(CFLAGS) -o $@ -c $< $(LFLAGS)

$(OBJDIR)/%.o: $(SRCDIR)/%.c | folders
	$(CC) $(CPPFLAGS) $(CFLAGS) -o $@ -c $<

.PHONY: folders
folders:
	@mkdir -p $(DIRS)
	
.PHONY: clean
clean:
	@rm -rf $(DIRS)


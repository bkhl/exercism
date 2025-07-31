class Badge {
    public String print(Integer id, String name, String department) {
        String id_prefix;

        if (id == null) {
            id_prefix = "";
        } else {
            id_prefix = "["
                    .concat(id.toString())
                    .concat("] - ");
        }

        if (department == null) {
            department = "Owner";
        }

        return id_prefix
                .concat(name)
                .concat(" - ")
                .concat(department.toUpperCase());
    }
}
